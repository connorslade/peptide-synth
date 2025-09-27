use engine::{memory::MemoryKey, memory_key};

pub const SCREEN: MemoryKey = memory_key!();

pub mod colors {
    use engine::color::Rgb;

    pub const BACKGROUND: Rgb<f32> = Rgb::hex(0x222034);
}
