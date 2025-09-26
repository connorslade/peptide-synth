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
        AMINO_R,
        AMINO_L,
        AMINO_P,

        SELECTED,
        GHOST,

        CONNECTOR_H,
        CONNECTOR_V
    }
}

macro include_asset($name:expr) {
    include_bytes!(concat!("../assets/", $name))
}

macro include_atlas($name:expr) {
    image::load_from_memory(include_asset!($name))
        .unwrap()
        .to_rgba8()
}

pub fn init(assets: &mut AssetConstructor) {
    let tiles = assets.register_atlas(include_atlas!("tiles.png"));
    assets.register_sprite(tiles, AMINO_R, (0, 0), (10, 11));
    assets.register_sprite(tiles, AMINO_L, (11, 0), (10, 11));
    assets.register_sprite(tiles, AMINO_P, (22, 0), (10, 11));

    assets.register_sprite(tiles, SELECTED, (22, 12), (10, 11));
    assets.register_sprite(tiles, GHOST, (11, 12), (10, 11));

    assets.register_sprite(tiles, CONNECTOR_H, (0, 12), (2, 2));
    assets.register_sprite(tiles, CONNECTOR_V, (3, 12), (2, 2));

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
