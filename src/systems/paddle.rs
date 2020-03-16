use amethyst::core::{SystemDesc, transform::Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>);

    fn run(&mut self, (mut transform_storage, paddle_storage, input_handler): Self::SystemData) {
        for (transform, paddle) in (&mut transform_storage, &paddle_storage).join() {
            let movement = match paddle.side {
                Side::Left => input_handler.axis_value("left_paddle"),
                Side::Right => input_handler.axis_value("right_paddle")
            };

            if let Some(movement) = movement {
                if movement.abs() > 0.0 {
                    transform.prepend_translation_y(movement);
                }
            }
        }
    }
}