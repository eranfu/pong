use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{math::Vector2, transform::Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::audio::{play_bounce_sound, Sounds};
use crate::pong::{ARENA_HEIGHT, Ball, Paddle, Side};

pub struct BounceBallSystem;

impl<'s> System<'s> for BounceBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        ReadExpect<'s, Sounds>,
        Read<'s, AssetStorage::<Source>>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (mut ball_storage, transform_storage, paddle_storage, sounds, source_storage, output): Self::SystemData) {
        for (ball, ball_transform) in (&mut ball_storage, &transform_storage).join() {
            let ball_translation = ball_transform.translation();
            if (ball_translation.y >= ARENA_HEIGHT - ball.radius && ball.velocity.y > 0.0) ||
                (ball_translation.y <= ball.radius && ball.velocity.y < 0.0) {
                ball.velocity.y = -ball.velocity.y;
                play_bounce_sound(&sounds, &source_storage, output.as_ref().map(|o| o.deref()));
            }

            for (paddle_transform, paddle) in (&transform_storage, &paddle_storage).join() {
                let paddle_translation = paddle_transform.translation();
                if is_in_rect(
                    &Vector2::new(ball_translation.x, ball_translation.y),
                    &Vector2::new(paddle_translation.x - 0.5 * paddle.width - ball.radius, paddle_translation.y - 0.5 * paddle.height - ball.radius),
                    &Vector2::new(paddle_translation.x + 0.5 * paddle.width + ball.radius, paddle_translation.y + 0.5 * paddle.height + ball.radius),
                ) {
                    if match paddle.side {
                        Side::Left => ball.velocity.x < 0.0,
                        Side::Right => ball.velocity.x > 0.0
                    } {
                        ball.velocity.x = -ball.velocity.x;
                        play_bounce_sound(&sounds, &source_storage, output.as_ref().map(|o| o.deref()));
                    }
                }
            }
        }
    }
}

fn is_in_rect(point: &Vector2<f32>, rect_min: &Vector2<f32>, rect_max: &Vector2<f32>) -> bool {
    return point.x >= rect_min.x && point.x <= rect_max.x &&
        point.y >= rect_min.y && point.y <= rect_max.y;
}