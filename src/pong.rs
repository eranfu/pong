use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
        let sprite_sheet = load_sprite_sheet(world);
        initialize_paddles(world, sprite_sheet.clone(), 0);
        initialize_ball(world, sprite_sheet, 1);
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
    pub side: Side,
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

pub struct Ball {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            velocity: Vector2::new(BALL_VELOCITY_X, BALL_VELOCITY_Y),
            radius: BALL_RADIUS,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>, sprite_number: usize) {
    world.create_entity()
        .with(Ball::default())
        .with({
            let mut transform = Transform::default();
            transform.set_translation_x(ARENA_WIDTH / 2.0);
            transform.set_translation_y(ARENA_HEIGHT / 2.0);
            transform
        })
        .with(SpriteRender {
            sprite_sheet,
            sprite_number,
        })
        .build();
}
