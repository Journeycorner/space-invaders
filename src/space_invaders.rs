use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
};

const ARENA_HEIGHT: f32 = 300.0;
const ARENA_WIDTH: f32 = 250.0;

const SPACECRAFT_HEIGHT: f32 = 17.0;
const SPACECRAFT_WIDTH: f32 = 26.0;

pub struct SpaceInvaders;

impl SimpleState for SpaceInvaders {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Spacecraft>();

        initialise_spacecraft(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

struct Spacecraft {
    pub width: f32,
    pub height: f32,
}

impl Spacecraft {
    fn new() -> Spacecraft {
        Spacecraft {
            width: SPACECRAFT_WIDTH,
            height: SPACECRAFT_HEIGHT,
        }
    }
}

impl Component for Spacecraft {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/space_invaders_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/space_invaders_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

/// Initialise the camera.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

/// Initialises initialise_spacecraft.
fn initialise_spacecraft(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // Assign the sprites for the spacecraft
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let mut left_transform = Transform::default();
    left_transform.set_xyz(ARENA_WIDTH / 2.0, SPACECRAFT_HEIGHT / 2.0, 0.0);

    // Create the spacecraft.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Spacecraft::new())
        .with(left_transform)
        .build();
}
