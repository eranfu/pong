use amethyst::core::{SystemDesc, transform::Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{ARENA_HEIGHT, Paddle, PADDLE_HEIGHT, Side};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>);

    fn run(&mut self, (mut transform_storage, paddle_storage, input_handler): Self::SystemData) {
        for (transform, paddle) in (&mut transform_storage, &paddle_storage).join() {
            let transform = transform as &mut Transform;
            let movement = match paddle.side {
                Side::Left => input_handler.axis_value("left_paddle"),
                Side::Right => input_handler.axis_value("right_paddle")
            };

            if let Some(movement) = movement {
                let movement = 1.2_f32 * movement;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + movement)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5_f32)
                        .max(PADDLE_HEIGHT * 0.5_f32));
            }
        }
    }
}