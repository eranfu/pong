use amethyst::core::{Time, transform::Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::pong::Ball;

pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>
    );

    fn run(&mut self, (ball_storage, mut transform_storage, time): Self::SystemData) {
        for (ball, transform) in (&ball_storage, &mut transform_storage).join() {
            let translation = transform.translation_mut();
            translation.x += ball.velocity.x * time.delta_seconds();
            translation.y += ball.velocity.y * time.delta_seconds();
        }
    }
}