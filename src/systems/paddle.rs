use amethyst::core::{SystemDesc, Time, transform::Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{ARENA_HEIGHT, Paddle, PADDLE_SPEED, Side};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transform_storage, paddle_storage, input_handler, time): Self::SystemData) {
        for (transform, paddle) in (&mut transform_storage, &paddle_storage).join() {
            let transform = transform as &mut Transform;
            let movement = match paddle.side {
                Side::Left => input_handler.axis_value("left_paddle"),
                Side::Right => input_handler.axis_value("right_paddle")
            };

            if let Some(movement) = movement {
                let movement = movement * PADDLE_SPEED * time.delta_seconds();
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + movement)
                        .min(ARENA_HEIGHT - paddle.height * 0.5_f32)
                        .max(paddle.height * 0.5_f32));
            }
        }
    }
}