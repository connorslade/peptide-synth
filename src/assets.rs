use engine::{
    assets::{FontRef, SpriteRef, constructor::AssetConstructor, font::FontDescriptor},
    define_refs,
};
use image::RgbaImage;

define_refs! {
    FontRef => {
        UNDEAD_FONT
    },
    SpriteRef => {
        CAMPAIGN_BUTTON,
        QUIT_BUTTON,

        SCORE_BAR,
        SCORE_ARROW,

        AMINO_R,
        AMINO_L,
        AMINO_A,
        AMINO_C,
        AMINO_D,
        AMINO_F,

        SELECTED,
        GHOST,

        CONNECTOR_H,
        CONNECTOR_V
    }
}

pub macro include_asset($name:expr) {
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $name))
}

macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub fn init(assets: &mut AssetConstructor) {
    let tiles = assets.register_atlas(include_atlas!("tiles.png"));
    assets.register_sprite(tiles, CAMPAIGN_BUTTON, (33, 0), (44, 12));
    assets.register_sprite(tiles, QUIT_BUTTON, (33, 13), (23, 12));

    assets.register_sprite(tiles, SCORE_BAR, (0, 36), (60, 7));
    assets.register_sprite(tiles, SCORE_ARROW, (0, 31), (5, 4));

    assets.register_sprite(tiles, AMINO_R, (0, 0), (10, 11));
    assets.register_sprite(tiles, AMINO_L, (11, 0), (10, 11));
    assets.register_sprite(tiles, AMINO_A, (22, 0), (10, 11));
    assets.register_sprite(tiles, AMINO_C, (0, 12), (10, 11));
    assets.register_sprite(tiles, AMINO_D, (11, 12), (10, 11));
    assets.register_sprite(tiles, AMINO_F, (22, 12), (10, 11));

    assets.register_sprite(tiles, SELECTED, (22, 24), (10, 11));
    assets.register_sprite(tiles, GHOST, (11, 24), (10, 11));

    assets.register_sprite(tiles, CONNECTOR_H, (0, 24), (2, 2));
    assets.register_sprite(tiles, CONNECTOR_V, (3, 24), (2, 2));

    load_font(
        assets,
        UNDEAD_FONT,
        include_atlas!("fonts/undead-pixel-11.png"),
        include_asset!("fonts/undead-pixel-11.ron"),
    );
}

fn load_font(assets: &mut AssetConstructor, asset: FontRef, atlas: RgbaImage, descriptor: &[u8]) {
    let font = assets.register_atlas(atlas);
    let descriptor = ron::de::from_bytes::<FontDescriptor>(descriptor).unwrap();
    assets.register_font(font, asset, descriptor);
}
