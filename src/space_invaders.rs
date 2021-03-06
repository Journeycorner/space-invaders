use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
};

pub const ARENA_HEIGHT: f32 = 640.0;
const ARENA_WIDTH: f32 = 480.0;

pub const SPACECRAFT_HEIGHT: f32 = 17.0;
const SPACECRAFT_WIDTH: f32 = 26.0;

pub const JELLY_FISH_ALIEN_HEIGHT: f32 = 17.0;
const JELLY_FISH_ALIEN_WIDTH: f32 = 26.0;

pub const MONSTER_ALIEN_HEIGHT: f32 = 17.0;
const MONSTER_ALIEN_WIDTH: f32 = 26.0;

pub const MUSHROOM_ALIEN_HEIGHT: f32 = 17.0;
const MUSHROOM_ALIEN_WIDTH: f32 = 26.0;

pub struct SpaceInvaders;

impl SimpleState for SpaceInvaders {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<JellyFishAlien>();
        world.register::<MonsterAlien>();
        world.register::<MushroomAlien>();

        initialise_spacecraft(world, sprite_sheet_handle.clone());
        initialise_aliens(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

pub struct Spacecraft {
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

pub struct JellyFishAlien {
    pub width: f32,
    pub height: f32,
    pub time_since_move: f32,
}

impl JellyFishAlien {
    fn new() -> JellyFishAlien {
        JellyFishAlien {
            width: JELLY_FISH_ALIEN_HEIGHT,
            height: JELLY_FISH_ALIEN_WIDTH,
            time_since_move: 0.0,
        }
    }
}

impl Component for JellyFishAlien {
    type Storage = DenseVecStorage<Self>;
}

pub struct MonsterAlien {
    pub width: f32,
    pub height: f32,
    pub time_since_move: f32,
}

impl MonsterAlien {
    fn new() -> MonsterAlien {
        MonsterAlien {
            width: JELLY_FISH_ALIEN_HEIGHT,
            height: JELLY_FISH_ALIEN_WIDTH,
            time_since_move: 0.0,
        }
    }
}

impl Component for MonsterAlien {
    type Storage = DenseVecStorage<Self>;
}

pub struct MushroomAlien {
    pub width: f32,
    pub height: f32,
    pub time_since_move: f32,
}

impl MushroomAlien {
    fn new() -> MushroomAlien {
        MushroomAlien {
            width: MUSHROOM_ALIEN_HEIGHT,
            height: MUSHROOM_ALIEN_WIDTH,
            time_since_move: 0.0,
        }
    }
}

impl Component for MushroomAlien {
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
        sprite_number: 0, // spacecraft is the first sprite in the sprite_sheet
    };

    let mut left_transform = Transform::default();
    left_transform.set_xyz(ARENA_WIDTH / 2.0, SPACECRAFT_HEIGHT / 2.0, 0.0);

    // Create the spacecraft.
    world
        .create_entity()
        .with(sprite_render)
        .with(Spacecraft::new())
        .with(left_transform)
        .build();
}

// /// Initialises initialise_spacecraft.
fn initialise_aliens(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    // Create the jelly fish aliens.
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };
    let mut x = 30.0;
    let mut y = 400.0;
    for i in 0..11 {
        let mut left_transform = Transform::default();
        left_transform.set_xyz(x, y, 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(JellyFishAlien::new())
            .with(left_transform)
            .build();
        x += 27.0;
    }

    // Create the monster aliens.
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 2,
    };
    let mut x = 30.0;
    let mut y = 350.0;
    for i in 0..11 {
        let mut left_transform = Transform::default();
        left_transform.set_xyz(x, y, 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(MonsterAlien::new())
            .with(left_transform)
            .build();
        x += 27.0;
    }
    let mut x = 30.0;
    let mut y = 300.0;
    for i in 0..11 {
        let mut left_transform = Transform::default();
        left_transform.set_xyz(x, y, 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(MonsterAlien::new())
            .with(left_transform)
            .build();
        x += 27.0;
    }

    // Create the mushroom aliens.
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 3,
    };
    let mut x = 30.0;
    let mut y = 250.0;
    for i in 0..11 {
        let mut left_transform = Transform::default();
        left_transform.set_xyz(x, y, 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(MushroomAlien::new())
            .with(left_transform)
            .build();
        x += 27.0;
    }
    let mut x = 30.0;
    let mut y = 200.0;
    for i in 0..11 {
        let mut left_transform = Transform::default();
        left_transform.set_xyz(x, y, 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(MushroomAlien::new())
            .with(left_transform)
            .build();
        x += 27.0;
    }
}
