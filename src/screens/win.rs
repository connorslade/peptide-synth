use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    memory_key,
};

use crate::{
    assets::{EX, UNDEAD_FONT},
    consts::SCREEN,
    misc::button::ButtonExt,
    screens::Screen,
};

const WIN_MESSAGE: &str = "Congratulations, you were able to optimize all the necessary peptides, and the ship safely made it to the Helios sector. Turns out aliens do exist, cool.\n\nYour work saved the mission and many lives; great job.";

pub fn render(ctx: &mut GraphicsContext) {
    let mut quit = false;
    Sprite::new(EX)
        .scale(Vector2::repeat(4.0))
        .position(ctx.size() - Vector2::new(16.0, 25.0), Anchor::TopRight)
        .button(memory_key!())
        .on_click(ctx, || quit = true)
        .draw(ctx);
    quit.then(|| ctx.memory.insert(SCREEN, Screen::Title));

    Text::new(UNDEAD_FONT, WIN_MESSAGE)
        .scale(Vector2::repeat(4.0))
        .shadow(-Vector2::y(), Rgb::hex(0x5c5b6a))
        .position(ctx.center(), Anchor::Center)
        .max_width(630.0)
        .draw(ctx);
}
