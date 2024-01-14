use crate::model::ModelVertex;

pub const PLANE_VERTICES: &[ModelVertex] = &[
    ModelVertex {
        position: [-0.5, 0.0, 0.5],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    }, // Bottom left
    ModelVertex {
        position: [0.5, 0.0, 0.5],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    }, // Bottom right
    ModelVertex {
        position: [-0.5, 0.0, -0.5],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    }, // Top left
    ModelVertex {
        position: [0.5, 0.0, -0.5],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    }, // Top right
];

#[rustfmt::skip]
pub const PLANE_INDICES: &[u32] = &[
    0, 1, 2,
    1, 3, 2,
];
