use cgmath::*;

pub fn create_vertices(
    nr: usize,
    nc: usize,
) -> (Vec<[f32; 4]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
    let cpts = control_points();
    let mut vertices: Vec<[f32; 4]> = Vec::with_capacity(12);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(12);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(12);
    let mut indices: Vec<u32> = Vec::with_capacity(12);
    for i in 0..32 {
        let this_patch = &cpts[i];
        let (mut patch_vertices, mut patch_normals, mut patch_uvs, patch_indices) =
            tesselate_patch(this_patch, nr, nc);

        let base = vertices.len() as u32;
        vertices.append(&mut patch_vertices);
        normals.append(&mut patch_normals);
        uvs.append(&mut patch_uvs);
        for j in patch_indices {
            indices.push(base + j);
        }
    }
    (vertices, normals, uvs, indices)
}

fn tesselate_patch(
    cpts: &Vec<Vec<Point3<f32>>>,
    nr: usize,
    nc: usize,
) -> (Vec<[f32; 4]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
    let mut verts: Vec<[f32; 4]> = Vec::with_capacity(nr * nc);
    let mut norms: Vec<[f32; 3]> = Vec::with_capacity(nr * nc);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(nr * nc);

    for r in 0..nr {
        let v = r as f64 / (nr - 1) as f64;
        let v2 = v * v;
        let v3 = v * v2;
        let mv = 1.0 - v;
        let mv2 = mv * mv;
        let mv3 = mv * mv2;

        let vp = [mv3, 3.0 * mv2 * v, 3.0 * mv * v2, v3];
        let dv = [
            -3.0 + 6.0 * v - 3.0 * v2,
            3.0 * (1.0 - 4.0 * v + 3.0 * v2),
            3.0 * (2.0 * v - 3.0 * v2),
            3.0 * v2,
        ];

        for c in 0..nc {
            let u = c as f64 / (nc - 1) as f64;
            let u2 = u * u;
            let u3 = u * u2;
            let mu = 1.0 - u;
            let mu2 = mu * mu;
            let mu3 = mu * mu2;

            let up = [mu3, 3.0 * mu2 * u, 3.0 * mu * u2, u3];
            let du = [
                -3.0 + 6.0 * u - 3.0 * u2,
                3.0 * (1.0 - 4.0 * u + 3.0 * u2),
                3.0 * (2.0 * u - 3.0 * u2),
                3.0 * u2,
            ];

            let w = [
                up[0] * vp[0],
                up[1] * vp[0],
                up[2] * vp[0],
                up[3] * vp[0],
                up[0] * vp[1],
                up[1] * vp[1],
                up[2] * vp[1],
                up[3] * vp[1],
                up[0] * vp[2],
                up[1] * vp[2],
                up[2] * vp[2],
                up[3] * vp[2],
                up[0] * vp[3],
                up[1] * vp[3],
                up[2] * vp[3],
                up[3] * vp[3],
            ];

            let dwdv = [
                du[0] * vp[0],
                du[1] * vp[0],
                du[2] * vp[0],
                du[3] * vp[0],
                du[0] * vp[1],
                du[1] * vp[1],
                du[2] * vp[1],
                du[3] * vp[1],
                du[0] * vp[2],
                du[1] * vp[2],
                du[2] * vp[2],
                du[3] * vp[2],
                du[0] * vp[3],
                du[1] * vp[3],
                du[2] * vp[3],
                du[3] * vp[3],
            ];

            let dwdu = [
                up[0] * dv[0],
                up[1] * dv[0],
                up[2] * dv[0],
                up[3] * dv[0],
                up[0] * dv[1],
                up[1] * dv[1],
                up[2] * dv[1],
                up[3] * dv[1],
                up[0] * dv[2],
                up[1] * dv[2],
                up[2] * dv[2],
                up[3] * dv[2],
                up[0] * dv[3],
                up[1] * dv[3],
                up[2] * dv[3],
                up[3] * dv[3],
            ];

            let mut pt = Vector3::new(0f64, 0f64, 0f64);
            let mut tan1 = Vector3::new(0f64, 0f64, 0f64);
            let mut tan2 = Vector3::new(0f64, 0f64, 0f64);
            for a in 0..4 {
                for b in 0..4 {
                    let index = 4 * a + b;

                    let cpt = Vector3::new(
                        cpts[a][b].x as f64,
                        cpts[a][b].y as f64,
                        cpts[a][b].z as f64,
                    );

                    pt += w[index] * cpt;
                    tan1 += dwdv[index] * cpt;
                    tan2 += dwdu[index] * cpt;
                }
            }

            verts.push([pt[0] as f32, pt[1] as f32, pt[2] as f32, 1.0]);
            let normal = tan1.normalize().cross(tan2.normalize());
            norms.push([normal[0] as f32, normal[1] as f32, normal[2] as f32]);
            let uv = [u as f32, v as f32];
            uvs.push(uv);
        }
    }

    let mut indices: Vec<u32> = Vec::with_capacity(2 * 3 * (nr - 1) * (nc - 1));
    for r in 0..(nr - 1) {
        for c in 0..(nc - 1) {
            indices.push((r * nc + c) as u32);
            indices.push((r * nc + c + 1) as u32);
            indices.push(((r + 1) * nc + c + 1) as u32);
            indices.push((r * nc + c) as u32);
            indices.push(((r + 1) * nc + c + 1) as u32);
            indices.push(((r + 1) * nc + c) as u32);
        }
    }

    (verts, norms, uvs, indices)
}

fn control_points() -> Vec<Vec<Vec<Point3<f32>>>> {
    [
        // 0
        vec![
            vec![
                Point3::new(1.4, 0.0, 2.4),
                Point3::new(1.4, -0.784, 2.4),
                Point3::new(0.784, -1.4, 2.4),
                Point3::new(0.0, -1.4, 2.4),
            ],
            vec![
                Point3::new(1.3375, 0.0, 2.53125),
                Point3::new(1.3375, -0.749, 2.53125),
                Point3::new(0.749, -1.3375, 2.53125),
                Point3::new(0.0, -1.3375, 2.53125),
            ],
            vec![
                Point3::new(1.4375, 0.0, 2.53125),
                Point3::new(1.4375, -0.805, 2.53125),
                Point3::new(0.805, -1.4375, 2.53125),
                Point3::new(0.0, -1.4375, 2.53125),
            ],
            vec![
                Point3::new(1.5, 0.0, 2.4),
                Point3::new(1.5, -0.84, 2.4),
                Point3::new(0.84, -1.5, 2.4),
                Point3::new(0.0, -1.5, 2.4),
            ],
        ],
        // 1
        vec![
            vec![
                Point3::new(0.0, -1.4, 2.4),
                Point3::new(-0.784, -1.4, 2.4),
                Point3::new(-1.4, -0.784, 2.4),
                Point3::new(-1.4, 0.0, 2.4),
            ],
            vec![
                Point3::new(0.0, -1.3375, 2.53125),
                Point3::new(-0.749, -1.3375, 2.53125),
                Point3::new(-1.3375, -0.749, 2.53125),
                Point3::new(-1.3375, 0.0, 2.53125),
            ],
            vec![
                Point3::new(0.0, -1.4375, 2.53125),
                Point3::new(-0.805, -1.4375, 2.53125),
                Point3::new(-1.4375, -0.805, 2.53125),
                Point3::new(-1.4375, 0.0, 2.53125),
            ],
            vec![
                Point3::new(0.0, -1.5, 2.4),
                Point3::new(-0.84, -1.5, 2.4),
                Point3::new(-1.5, -0.84, 2.4),
                Point3::new(-1.5, 0.0, 2.4),
            ],
        ],
        // 2
        vec![
            vec![
                Point3::new(-1.4, 0.0, 2.4),
                Point3::new(-1.4, 0.784, 2.4),
                Point3::new(-0.784, 1.4, 2.4),
                Point3::new(0.0, 1.4, 2.4),
            ],
            vec![
                Point3::new(-1.3375, 0.0, 2.53125),
                Point3::new(-1.3375, 0.749, 2.53125),
                Point3::new(-0.749, 1.3375, 2.53125),
                Point3::new(0.0, 1.3375, 2.53125),
            ],
            vec![
                Point3::new(-1.4375, 0.0, 2.53125),
                Point3::new(-1.4375, 0.805, 2.53125),
                Point3::new(-0.805, 1.4375, 2.53125),
                Point3::new(0.0, 1.4375, 2.53125),
            ],
            vec![
                Point3::new(-1.5, 0.0, 2.4),
                Point3::new(-1.5, 0.84, 2.4),
                Point3::new(-0.84, 1.5, 2.4),
                Point3::new(0.0, 1.5, 2.4),
            ],
        ],
        // 3
        vec![
            vec![
                Point3::new(0.0, 1.4, 2.4),
                Point3::new(0.784, 1.4, 2.4),
                Point3::new(1.4, 0.784, 2.4),
                Point3::new(1.4, 0.0, 2.4),
            ],
            vec![
                Point3::new(0.0, 1.3375, 2.53125),
                Point3::new(0.749, 1.3375, 2.53125),
                Point3::new(1.3375, 0.749, 2.53125),
                Point3::new(1.3375, 0.0, 2.53125),
            ],
            vec![
                Point3::new(0.0, 1.4375, 2.53125),
                Point3::new(0.805, 1.4375, 2.53125),
                Point3::new(1.4375, 0.805, 2.53125),
                Point3::new(1.4375, 0.0, 2.53125),
            ],
            vec![
                Point3::new(0.0, 1.5, 2.4),
                Point3::new(0.84, 1.5, 2.4),
                Point3::new(1.5, 0.84, 2.4),
                Point3::new(1.5, 0.0, 2.4),
            ],
        ],
        // 4
        vec![
            vec![
                Point3::new(1.5, 0.0, 2.4),
                Point3::new(1.5, -0.84, 2.4),
                Point3::new(0.84, -1.5, 2.4),
                Point3::new(0.0, -1.5, 2.4),
            ],
            vec![
                Point3::new(1.75, 0.0, 1.875),
                Point3::new(1.75, -0.98, 1.875),
                Point3::new(0.98, -1.75, 1.875),
                Point3::new(0.0, -1.75, 1.875),
            ],
            vec![
                Point3::new(2.0, 0.0, 1.35),
                Point3::new(2.0, -1.12, 1.35),
                Point3::new(1.12, -2.0, 1.35),
                Point3::new(0.0, -2.0, 1.35),
            ],
            vec![
                Point3::new(2.0, 0.0, 0.9),
                Point3::new(2.0, -1.12, 0.9),
                Point3::new(1.12, -2.0, 0.9),
                Point3::new(0.0, -2.0, 0.9),
            ],
        ],
        // 5
        vec![
            vec![
                Point3::new(0.0, -1.5, 2.4),
                Point3::new(-0.84, -1.5, 2.4),
                Point3::new(-1.5, -0.84, 2.4),
                Point3::new(-1.5, 0.0, 2.4),
            ],
            vec![
                Point3::new(0.0, -1.75, 1.875),
                Point3::new(-0.98, -1.75, 1.875),
                Point3::new(-1.75, -0.98, 1.875),
                Point3::new(-1.75, 0.0, 1.875),
            ],
            vec![
                Point3::new(0.0, -2.0, 1.35),
                Point3::new(-1.12, -2.0, 1.35),
                Point3::new(-2.0, -1.12, 1.35),
                Point3::new(-2.0, 0.0, 1.35),
            ],
            vec![
                Point3::new(0.0, -2.0, 0.9),
                Point3::new(-1.12, -2.0, 0.9),
                Point3::new(-2.0, -1.12, 0.9),
                Point3::new(-2.0, 0.0, 0.9),
            ],
        ],
        // 6
        vec![
            vec![
                Point3::new(-1.5, 0.0, 2.4),
                Point3::new(-1.5, 0.84, 2.4),
                Point3::new(-0.84, 1.5, 2.4),
                Point3::new(0.0, 1.5, 2.4),
            ],
            vec![
                Point3::new(-1.75, 0.0, 1.875),
                Point3::new(-1.75, 0.98, 1.875),
                Point3::new(-0.98, 1.75, 1.875),
                Point3::new(0.0, 1.75, 1.875),
            ],
            vec![
                Point3::new(-2.0, 0.0, 1.35),
                Point3::new(-2.0, 1.12, 1.35),
                Point3::new(-1.12, 2.0, 1.35),
                Point3::new(0.0, 2.0, 1.35),
            ],
            vec![
                Point3::new(-2.0, 0.0, 0.9),
                Point3::new(-2.0, 1.12, 0.9),
                Point3::new(-1.12, 2.0, 0.9),
                Point3::new(0.0, 2.0, 0.9),
            ],
        ],
        // 7
        vec![
            vec![
                Point3::new(0.0, 1.5, 2.4),
                Point3::new(0.84, 1.5, 2.4),
                Point3::new(1.5, 0.84, 2.4),
                Point3::new(1.5, 0.0, 2.4),
            ],
            vec![
                Point3::new(0.0, 1.75, 1.875),
                Point3::new(0.98, 1.75, 1.875),
                Point3::new(1.75, 0.98, 1.875),
                Point3::new(1.75, 0.0, 1.875),
            ],
            vec![
                Point3::new(0.0, 2.0, 1.35),
                Point3::new(1.12, 2.0, 1.35),
                Point3::new(2.0, 1.12, 1.35),
                Point3::new(2.0, 0.0, 1.35),
            ],
            vec![
                Point3::new(0.0, 2.0, 0.9),
                Point3::new(1.12, 2.0, 0.9),
                Point3::new(2.0, 1.12, 0.9),
                Point3::new(2.0, 0.0, 0.9),
            ],
        ],
        // 8
        vec![
            vec![
                Point3::new(2.0, 0.0, 0.9),
                Point3::new(2.0, -1.12, 0.9),
                Point3::new(1.12, -2.0, 0.9),
                Point3::new(0.0, -2.0, 0.9),
            ],
            vec![
                Point3::new(2.0, 0.0, 0.45),
                Point3::new(2.0, -1.12, 0.45),
                Point3::new(1.12, -2.0, 0.45),
                Point3::new(0.0, -2.0, 0.45),
            ],
            vec![
                Point3::new(1.5, 0.0, 0.225),
                Point3::new(1.5, -0.84, 0.225),
                Point3::new(0.84, -1.5, 0.225),
                Point3::new(0.0, -1.5, 0.225),
            ],
            vec![
                Point3::new(1.5, 0.0, 0.15),
                Point3::new(1.5, -0.84, 0.15),
                Point3::new(0.84, -1.5, 0.15),
                Point3::new(0.0, -1.5, 0.15),
            ],
        ],
        // 9
        vec![
            vec![
                Point3::new(0.0, -2.0, 0.9),
                Point3::new(-1.12, -2.0, 0.9),
                Point3::new(-2.0, -1.12, 0.9),
                Point3::new(-2.0, 0.0, 0.9),
            ],
            vec![
                Point3::new(0.0, -2.0, 0.45),
                Point3::new(-1.12, -2.0, 0.45),
                Point3::new(-2.0, -1.12, 0.45),
                Point3::new(-2.0, 0.0, 0.45),
            ],
            vec![
                Point3::new(0.0, -1.5, 0.225),
                Point3::new(-0.84, -1.5, 0.225),
                Point3::new(-1.5, -0.84, 0.225),
                Point3::new(-1.5, 0.0, 0.225),
            ],
            vec![
                Point3::new(0.0, -1.5, 0.15),
                Point3::new(-0.84, -1.5, 0.15),
                Point3::new(-1.5, -0.84, 0.15),
                Point3::new(-1.5, 0.0, 0.15),
            ],
        ],
        // 10
        vec![
            vec![
                Point3::new(-2.0, 0.0, 0.9),
                Point3::new(-2.0, 1.12, 0.9),
                Point3::new(-1.12, 2.0, 0.9),
                Point3::new(0.0, 2.0, 0.9),
            ],
            vec![
                Point3::new(-2.0, 0.0, 0.45),
                Point3::new(-2.0, 1.12, 0.45),
                Point3::new(-1.12, 2.0, 0.45),
                Point3::new(0.0, 2.0, 0.45),
            ],
            vec![
                Point3::new(-1.5, 0.0, 0.225),
                Point3::new(-1.5, 0.84, 0.225),
                Point3::new(-0.84, 1.5, 0.225),
                Point3::new(0.0, 1.5, 0.225),
            ],
            vec![
                Point3::new(-1.5, 0.0, 0.15),
                Point3::new(-1.5, 0.84, 0.15),
                Point3::new(-0.84, 1.5, 0.15),
                Point3::new(0.0, 1.5, 0.15),
            ],
        ],
        // 11
        vec![
            vec![
                Point3::new(0.0, 2.0, 0.9),
                Point3::new(1.12, 2.0, 0.9),
                Point3::new(2.0, 1.12, 0.9),
                Point3::new(2.0, 0.0, 0.9),
            ],
            vec![
                Point3::new(0.0, 2.0, 0.45),
                Point3::new(1.12, 2.0, 0.45),
                Point3::new(2.0, 1.12, 0.45),
                Point3::new(2.0, 0.0, 0.45),
            ],
            vec![
                Point3::new(0.0, 1.5, 0.225),
                Point3::new(0.84, 1.5, 0.225),
                Point3::new(1.5, 0.84, 0.225),
                Point3::new(1.5, 0.0, 0.225),
            ],
            vec![
                Point3::new(0.0, 1.5, 0.15),
                Point3::new(0.84, 1.5, 0.15),
                Point3::new(1.5, 0.84, 0.15),
                Point3::new(1.5, 0.0, 0.15),
            ],
        ],
        // 12
        vec![
            vec![
                Point3::new(-1.6, 0.0, 2.025),
                Point3::new(-1.6, -0.3, 2.025),
                Point3::new(-1.5, -0.3, 2.25),
                Point3::new(-1.5, 0.0, 2.25),
            ],
            vec![
                Point3::new(-2.3, 0.0, 2.025),
                Point3::new(-2.3, -0.3, 2.025),
                Point3::new(-2.5, -0.3, 2.25),
                Point3::new(-2.5, 0.0, 2.25),
            ],
            vec![
                Point3::new(-2.7, 0.0, 2.025),
                Point3::new(-2.7, -0.3, 2.025),
                Point3::new(-3.0, -0.3, 2.25),
                Point3::new(-3.0, 0.0, 2.25),
            ],
            vec![
                Point3::new(-2.7, 0.0, 1.8),
                Point3::new(-2.7, -0.3, 1.8),
                Point3::new(-3.0, -0.3, 1.8),
                Point3::new(-3.0, 0.0, 1.8),
            ],
        ],
        // 13
        vec![
            vec![
                Point3::new(-1.5, 0.0, 2.25),
                Point3::new(-1.5, 0.3, 2.25),
                Point3::new(-1.6, 0.3, 2.025),
                Point3::new(-1.6, 0.0, 2.025),
            ],
            vec![
                Point3::new(-2.5, 0.0, 2.25),
                Point3::new(-2.5, 0.3, 2.25),
                Point3::new(-2.3, 0.3, 2.025),
                Point3::new(-2.3, 0.0, 2.025),
            ],
            vec![
                Point3::new(-3.0, 0.0, 2.25),
                Point3::new(-3.0, 0.3, 2.25),
                Point3::new(-2.7, 0.3, 2.025),
                Point3::new(-2.7, 0.0, 2.025),
            ],
            vec![
                Point3::new(-3.0, 0.0, 1.8),
                Point3::new(-3.0, 0.3, 1.8),
                Point3::new(-2.7, 0.3, 1.8),
                Point3::new(-2.7, 0.0, 1.8),
            ],
        ],
        // 14
        vec![
            vec![
                Point3::new(-2.7, 0.0, 1.8),
                Point3::new(-2.7, -0.3, 1.8),
                Point3::new(-3.0, -0.3, 1.8),
                Point3::new(-3.0, 0.0, 1.8),
            ],
            vec![
                Point3::new(-2.7, 0.0, 1.575),
                Point3::new(-2.7, -0.3, 1.575),
                Point3::new(-3.0, -0.3, 1.35),
                Point3::new(-3.0, 0.0, 1.35),
            ],
            vec![
                Point3::new(-2.5, 0.0, 1.125),
                Point3::new(-2.5, -0.3, 1.125),
                Point3::new(-2.65, -0.3, 0.9375),
                Point3::new(-2.65, 0.0, 0.9375),
            ],
            vec![
                Point3::new(-2.0, 0.0, 0.9),
                Point3::new(-2.0, -0.3, 0.9),
                Point3::new(-1.9, -0.3, 0.6),
                Point3::new(-1.9, 0.0, 0.6),
            ],
        ],
        // 15
        vec![
            vec![
                Point3::new(-3.0, 0.0, 1.8),
                Point3::new(-3.0, 0.3, 1.8),
                Point3::new(-2.7, 0.3, 1.8),
                Point3::new(-2.7, 0.0, 1.8),
            ],
            vec![
                Point3::new(-3.0, 0.0, 1.35),
                Point3::new(-3.0, 0.3, 1.35),
                Point3::new(-2.7, 0.3, 1.575),
                Point3::new(-2.7, 0.0, 1.575),
            ],
            vec![
                Point3::new(-2.65, 0.0, 0.9375),
                Point3::new(-2.65, 0.3, 0.9375),
                Point3::new(-2.5, 0.3, 1.125),
                Point3::new(-2.5, 0.0, 1.125),
            ],
            vec![
                Point3::new(-1.9, 0.0, 0.6),
                Point3::new(-1.9, 0.3, 0.6),
                Point3::new(-2.0, 0.3, 0.9),
                Point3::new(-2.0, 0.0, 0.9),
            ],
        ],
        // 16
        vec![
            vec![
                Point3::new(1.7, 0.0, 1.425),
                Point3::new(1.7, -0.66, 1.425),
                Point3::new(1.7, -0.66, 0.6),
                Point3::new(1.7, 0.0, 0.6),
            ],
            vec![
                Point3::new(2.6, 0.0, 1.425),
                Point3::new(2.6, -0.66, 1.425),
                Point3::new(3.1, -0.66, 0.825),
                Point3::new(3.1, 0.0, 0.825),
            ],
            vec![
                Point3::new(2.3, 0.0, 2.1),
                Point3::new(2.3, -0.25, 2.1),
                Point3::new(2.4, -0.25, 2.025),
                Point3::new(2.4, 0.0, 2.025),
            ],
            vec![
                Point3::new(2.7, 0.0, 2.4),
                Point3::new(2.7, -0.25, 2.4),
                Point3::new(3.3, -0.25, 2.4),
                Point3::new(3.3, 0.0, 2.4),
            ],
        ],
        // 17
        vec![
            vec![
                Point3::new(1.7, 0.0, 0.6),
                Point3::new(1.7, 0.66, 0.6),
                Point3::new(1.7, 0.66, 1.425),
                Point3::new(1.7, 0.0, 1.425),
            ],
            vec![
                Point3::new(3.1, 0.0, 0.825),
                Point3::new(3.1, 0.66, 0.825),
                Point3::new(2.6, 0.66, 1.425),
                Point3::new(2.6, 0.0, 1.425),
            ],
            vec![
                Point3::new(2.4, 0.0, 2.025),
                Point3::new(2.4, 0.25, 2.025),
                Point3::new(2.3, 0.25, 2.1),
                Point3::new(2.3, 0.0, 2.1),
            ],
            vec![
                Point3::new(3.3, 0.0, 2.4),
                Point3::new(3.3, 0.25, 2.4),
                Point3::new(2.7, 0.25, 2.4),
                Point3::new(2.7, 0.0, 2.4),
            ],
        ],
        // 18
        vec![
            vec![
                Point3::new(2.7, 0.0, 2.4),
                Point3::new(2.7, -0.25, 2.4),
                Point3::new(3.3, -0.25, 2.4),
                Point3::new(3.3, 0.0, 2.4),
            ],
            vec![
                Point3::new(2.8, 0.0, 2.475),
                Point3::new(2.8, -0.25, 2.475),
                Point3::new(3.525, -0.25, 2.49375),
                Point3::new(3.525, 0.0, 2.49375),
            ],
            vec![
                Point3::new(2.9, 0.0, 2.475),
                Point3::new(2.9, -0.15, 2.475),
                Point3::new(3.45, -0.15, 2.5125),
                Point3::new(3.45, 0.0, 2.5125),
            ],
            vec![
                Point3::new(2.8, 0.0, 2.4),
                Point3::new(2.8, -0.15, 2.4),
                Point3::new(3.2, -0.15, 2.4),
                Point3::new(3.2, 0.0, 2.4),
            ],
        ],
        // 19
        vec![
            vec![
                Point3::new(3.3, 0.0, 2.4),
                Point3::new(3.3, 0.25, 2.4),
                Point3::new(2.7, 0.25, 2.4),
                Point3::new(2.7, 0.0, 2.4),
            ],
            vec![
                Point3::new(3.525, 0.0, 2.49375),
                Point3::new(3.525, 0.25, 2.49375),
                Point3::new(2.8, 0.25, 2.475),
                Point3::new(2.8, 0.0, 2.475),
            ],
            vec![
                Point3::new(3.45, 0.0, 2.5125),
                Point3::new(3.45, 0.15, 2.5125),
                Point3::new(2.9, 0.15, 2.475),
                Point3::new(2.9, 0.0, 2.475),
            ],
            vec![
                Point3::new(3.2, 0.0, 2.4),
                Point3::new(3.2, 0.15, 2.4),
                Point3::new(2.8, 0.15, 2.4),
                Point3::new(2.8, 0.0, 2.4),
            ],
        ],
        // 20
        vec![
            vec![
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
            ],
            vec![
                Point3::new(0.8, 0.0, 3.15),
                Point3::new(0.8, -0.45, 3.15),
                Point3::new(0.45, -0.8, 3.15),
                Point3::new(0.0, -0.8, 3.15),
            ],
            vec![
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
            ],
            vec![
                Point3::new(0.2, 0.0, 2.7),
                Point3::new(0.2, -0.112, 2.7),
                Point3::new(0.112, -0.2, 2.7),
                Point3::new(0.0, -0.2, 2.7),
            ],
        ],
        // 21
        vec![
            vec![
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
            ],
            vec![
                Point3::new(0.0, -0.8, 3.15),
                Point3::new(-0.45, -0.8, 3.15),
                Point3::new(-0.8, -0.45, 3.15),
                Point3::new(-0.8, 0.0, 3.15),
            ],
            vec![
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
            ],
            vec![
                Point3::new(0.0, -0.2, 2.7),
                Point3::new(-0.112, -0.2, 2.7),
                Point3::new(-0.2, -0.112, 2.7),
                Point3::new(-0.2, 0.0, 2.7),
            ],
        ],
        // 22
        vec![
            vec![
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
            ],
            vec![
                Point3::new(-0.8, 0.0, 3.15),
                Point3::new(-0.8, 0.45, 3.15),
                Point3::new(-0.45, 0.8, 3.15),
                Point3::new(0.0, 0.8, 3.15),
            ],
            vec![
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
            ],
            vec![
                Point3::new(-0.2, 0.0, 2.7),
                Point3::new(-0.2, 0.112, 2.7),
                Point3::new(-0.112, 0.2, 2.7),
                Point3::new(0.0, 0.2, 2.7),
            ],
        ],
        // 23
        vec![
            vec![
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
                Point3::new(0.0, 0.0, 3.15),
            ],
            vec![
                Point3::new(0.0, 0.8, 3.15),
                Point3::new(0.45, 0.8, 3.15),
                Point3::new(0.8, 0.45, 3.15),
                Point3::new(0.8, 0.0, 3.15),
            ],
            vec![
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
                Point3::new(0.0, 0.0, 2.85),
            ],
            vec![
                Point3::new(0.0, 0.2, 2.7),
                Point3::new(0.112, 0.2, 2.7),
                Point3::new(0.2, 0.112, 2.7),
                Point3::new(0.2, 0.0, 2.7),
            ],
        ],
        // 24
        vec![
            vec![
                Point3::new(0.2, 0.0, 2.7),
                Point3::new(0.2, -0.112, 2.7),
                Point3::new(0.112, -0.2, 2.7),
                Point3::new(0.0, -0.2, 2.7),
            ],
            vec![
                Point3::new(0.4, 0.0, 2.55),
                Point3::new(0.4, -0.224, 2.55),
                Point3::new(0.224, -0.4, 2.55),
                Point3::new(0.0, -0.4, 2.55),
            ],
            vec![
                Point3::new(1.3, 0.0, 2.55),
                Point3::new(1.3, -0.728, 2.55),
                Point3::new(0.728, -1.3, 2.55),
                Point3::new(0.0, -1.3, 2.55),
            ],
            vec![
                Point3::new(1.3, 0.0, 2.4),
                Point3::new(1.3, -0.728, 2.4),
                Point3::new(0.728, -1.3, 2.4),
                Point3::new(0.0, -1.3, 2.4),
            ],
        ],
        // 25
        vec![
            vec![
                Point3::new(0.0, -0.2, 2.7),
                Point3::new(-0.112, -0.2, 2.7),
                Point3::new(-0.2, -0.112, 2.7),
                Point3::new(-0.2, 0.0, 2.7),
            ],
            vec![
                Point3::new(0.0, -0.4, 2.55),
                Point3::new(-0.224, -0.4, 2.55),
                Point3::new(-0.4, -0.224, 2.55),
                Point3::new(-0.4, 0.0, 2.55),
            ],
            vec![
                Point3::new(0.0, -1.3, 2.55),
                Point3::new(-0.728, -1.3, 2.55),
                Point3::new(-1.3, -0.728, 2.55),
                Point3::new(-1.3, 0.0, 2.55),
            ],
            vec![
                Point3::new(0.0, -1.3, 2.4),
                Point3::new(-0.728, -1.3, 2.4),
                Point3::new(-1.3, -0.728, 2.4),
                Point3::new(-1.3, 0.0, 2.4),
            ],
        ],
        // 26
        vec![
            vec![
                Point3::new(-0.2, 0.0, 2.7),
                Point3::new(-0.2, 0.112, 2.7),
                Point3::new(-0.112, 0.2, 2.7),
                Point3::new(0.0, 0.2, 2.7),
            ],
            vec![
                Point3::new(-0.4, 0.0, 2.55),
                Point3::new(-0.4, 0.224, 2.55),
                Point3::new(-0.224, 0.4, 2.55),
                Point3::new(0.0, 0.4, 2.55),
            ],
            vec![
                Point3::new(-1.3, 0.0, 2.55),
                Point3::new(-1.3, 0.728, 2.55),
                Point3::new(-0.728, 1.3, 2.55),
                Point3::new(0.0, 1.3, 2.55),
            ],
            vec![
                Point3::new(-1.3, 0.0, 2.4),
                Point3::new(-1.3, 0.728, 2.4),
                Point3::new(-0.728, 1.3, 2.4),
                Point3::new(0.0, 1.3, 2.4),
            ],
        ],
        // 27
        vec![
            vec![
                Point3::new(0.0, 0.2, 2.7),
                Point3::new(0.112, 0.2, 2.7),
                Point3::new(0.2, 0.112, 2.7),
                Point3::new(0.2, 0.0, 2.7),
            ],
            vec![
                Point3::new(0.0, 0.4, 2.55),
                Point3::new(0.224, 0.4, 2.55),
                Point3::new(0.4, 0.224, 2.55),
                Point3::new(0.4, 0.0, 2.55),
            ],
            vec![
                Point3::new(0.0, 1.3, 2.55),
                Point3::new(0.728, 1.3, 2.55),
                Point3::new(1.3, 0.728, 2.55),
                Point3::new(1.3, 0.0, 2.55),
            ],
            vec![
                Point3::new(0.0, 1.3, 2.4),
                Point3::new(0.728, 1.3, 2.4),
                Point3::new(1.3, 0.728, 2.4),
                Point3::new(1.3, 0.0, 2.4),
            ],
        ],
        // 28
        vec![
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
            ],
            vec![
                Point3::new(1.425, 0.0, 0.0),
                Point3::new(1.425, 0.798, 0.0),
                Point3::new(0.798, 1.425, 0.0),
                Point3::new(0.0, 1.425, 0.0),
            ],
            vec![
                Point3::new(1.5, 0.0, 0.075),
                Point3::new(1.5, 0.84, 0.075),
                Point3::new(0.84, 1.5, 0.075),
                Point3::new(0.0, 1.5, 0.075),
            ],
            vec![
                Point3::new(1.5, 0.0, 0.15),
                Point3::new(1.5, 0.84, 0.15),
                Point3::new(0.84, 1.5, 0.15),
                Point3::new(0.0, 1.5, 0.15),
            ],
        ],
        // 29
        vec![
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
            ],
            vec![
                Point3::new(0.0, 1.425, 0.0),
                Point3::new(-0.798, 1.425, 0.0),
                Point3::new(-1.425, 0.798, 0.0),
                Point3::new(-1.425, 0.0, 0.0),
            ],
            vec![
                Point3::new(0.0, 1.5, 0.075),
                Point3::new(-0.84, 1.5, 0.075),
                Point3::new(-1.5, 0.84, 0.075),
                Point3::new(-1.5, 0.0, 0.075),
            ],
            vec![
                Point3::new(0.0, 1.5, 0.15),
                Point3::new(-0.84, 1.5, 0.15),
                Point3::new(-1.5, 0.84, 0.15),
                Point3::new(-1.5, 0.0, 0.15),
            ],
        ],
        // 30
        vec![
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
            ],
            vec![
                Point3::new(-1.425, 0.0, 0.0),
                Point3::new(-1.425, -0.798, 0.0),
                Point3::new(-0.798, -1.425, 0.0),
                Point3::new(0.0, -1.425, 0.0),
            ],
            vec![
                Point3::new(-1.5, 0.0, 0.075),
                Point3::new(-1.5, -0.84, 0.075),
                Point3::new(-0.84, -1.5, 0.075),
                Point3::new(0.0, -1.5, 0.075),
            ],
            vec![
                Point3::new(-1.5, 0.0, 0.15),
                Point3::new(-1.5, -0.84, 0.15),
                Point3::new(-0.84, -1.5, 0.15),
                Point3::new(0.0, -1.5, 0.15),
            ],
        ],
        // 31
        vec![
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 0.0),
            ],
            vec![
                Point3::new(0.0, -1.425, 0.0),
                Point3::new(0.798, -1.425, 0.0),
                Point3::new(1.425, -0.798, 0.0),
                Point3::new(1.425, 0.0, 0.0),
            ],
            vec![
                Point3::new(0.0, -1.5, 0.075),
                Point3::new(0.84, -1.5, 0.075),
                Point3::new(1.5, -0.84, 0.075),
                Point3::new(1.5, 0.0, 0.075),
            ],
            vec![
                Point3::new(0.0, -1.5, 0.15),
                Point3::new(0.84, -1.5, 0.15),
                Point3::new(1.5, -0.84, 0.15),
                Point3::new(1.5, 0.0, 0.15),
            ],
        ],
    ]
    .to_vec()
}
