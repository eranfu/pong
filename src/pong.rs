use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub(crate) struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
        let sprite_sheet = load_sprite_sheet(world);
        initialize_paddles(world, sprite_sheet, 0);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(PartialEq)]
pub struct Paddle {
    side: Side,
    width: f32,
    height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>, sprite_number: usize) {
    let left = {
        let mut left = Transform::default();
        left.set_translation_xyz(PADDLE_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);
        left
    };

    let right = {
        let mut right = Transform::default();
        right.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);
        right
    };

    let sprite_renderer = SpriteRender {
        sprite_sheet,
        sprite_number,
    };

    world.register::<Paddle>();

    world.create_entity()
        .with(Paddle::new(Side::Left))
        .with(left)
        .with(sprite_renderer.clone())
        .build();

    world.create_entity()
        .with(Paddle::new(Side::Right))
        .with(right)
        .with(sprite_renderer)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load("texture/pong_spritesheet.png", ImageFormat::default(), (), &texture_storage)
    };

    let sprite_sheet = {
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load("texture/pong_spritesheet.ron", SpriteSheetFormat(texture), (), &sprite_sheet_storage)
    };

    sprite_sheet
}
