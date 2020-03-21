use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector2, Time, transform::Transform},
    ecs::Entity,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_SPEED: f32 = 60.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;

const BALL_VELOCITY_X: f32 = 75.0;
const BALL_VELOCITY_Y: f32 = 50.0;
const BALL_RADIUS: f32 = 2.0;

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.ball_spawn_timer.replace(1.0);
        self.sprite_sheet.replace(load_sprite_sheet(world));
        initialize_camera(world);
        let sprite_sheet = self.sprite_sheet.clone().unwrap();
        initialize_paddles(world, sprite_sheet.clone(), 0);
        initialize_score_board(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(timer) = self.ball_spawn_timer.take() {
            let timer = {
                let time = data.world.read_resource::<Time>();
                timer - time.delta_seconds()
            };
            if timer <= 0.0 {
                initialize_ball(&mut data.world, self.sprite_sheet.clone().unwrap(), 1);
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
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
    pub width: f32,
    pub height: f32,
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

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn initialize_score_board(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf", TtfFormat, (), &world.read_resource());

    let p1_score = world.create_entity()
        .with(UiTransform::new(
            "P1".into(),
            Anchor::TopMiddle, Anchor::TopMiddle,
            -50., -50., 1., 200., 50.))
        .with(UiText::new(font.clone(), "0".into(), [1., 1., 1., 1., ], 50.))
        .build();

    let p2_score = world.create_entity()
        .with(UiTransform::new(
            "P2".into(),
            Anchor::TopMiddle, Anchor::TopMiddle,
            50., -50., 1., 200., 50.))
        .with(UiText::new(font, "0".into(), [1., 1., 1., 1., ], 50.))
        .build();

    world.insert(ScoreText {
        p1_score,
        p2_score,
    });
}
