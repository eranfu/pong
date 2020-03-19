use amethyst::{
    core::transform::Transform,
    ecs::{Join, System, WriteStorage},
};

use crate::pong::{ARENA_WIDTH, Ball};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut ball_storage, mut transform_storage): Self::SystemData) {
        for (ball, transform) in (&mut ball_storage, &mut transform_storage).join() {
            let did_hit = {
                let ball_x = transform.translation().x;
                if ball_x <= ball.radius {
                    println!("hit left");
                    true
                } else if ball_x >= ARENA_WIDTH - ball.radius {
                    println!("hit right");
                    true
                } else {
                    false
                }
            };

            if did_hit {
                ball.velocity.x = -ball.velocity.x;
                transform.set_translation_x(ARENA_WIDTH / 2.0);
            }
        }
    }
}