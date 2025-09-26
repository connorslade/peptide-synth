use engine::{
    application::{Application, ApplicationArgs},
    drawable::{Anchor, Drawable, shape::circle::Circle},
    exports::winit::window::WindowAttributes,
};

fn main() {
    Application::new(ApplicationArgs {
        window_attributes: WindowAttributes::default(),
        asset_constructor: Box::new(|_| {}),
        resumed: Box::new(|| {
            Box::new(|ctx| {
                Circle::new(16.0)
                    .position(ctx.input.mouse(), Anchor::Center)
                    .draw(ctx);
            })
        }),
        multisample: None,
    })
    .run()
    .unwrap();
}
