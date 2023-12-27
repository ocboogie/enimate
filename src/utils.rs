use egui::{pos2, Pos2};

pub fn rotate_vec_around_0(vector: Pos2, angle: f32) -> Pos2 {
    let (sin, cos) = angle.sin_cos();
    pos2(
        vector.x * cos - vector.y * sin,
        vector.x * sin + vector.y * cos,
    )
}

pub fn rotate_vec_around_vector(vector: Pos2, angle: f32, around: Pos2) -> Pos2 {
    let (sin, cos) = angle.sin_cos();
    let vector = vector - around.to_vec2();
    let vector = pos2(
        vector.x * cos - vector.y * sin,
        vector.x * sin + vector.y * cos,
    );
    vector + around.to_vec2()
}
