use engine::exports::nalgebra::Vector2;

pub mod amino;
pub mod level;
pub mod peptide;

pub fn world_to_screen(world: Vector2<i32>) -> Vector2<f32> {
    world.map(|x| (x * 12 * 6) as f32)
}
