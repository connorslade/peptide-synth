use engine::exports::nalgebra::Vector2;

pub mod amino;
pub mod level;
pub mod peptide;

// pub fn screen_to_world(screen: Vector2<f32>) -> Vector2<i32> {
//     screen.map(|x| (x / 12.0 / 6.0).round() as i32)
// }

pub fn world_to_screen(world: Vector2<i32>) -> Vector2<f32> {
    world.map(|x| (x * 12 * 6) as f32)
}
