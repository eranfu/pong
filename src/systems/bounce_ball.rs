use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::pong::{ARENA_HEIGHT, Ball, Paddle, PADDLE_HEIGHT, PADDLE_WIDTH, Side};

pub struct BounceBallSystem;

impl<'s> System<'s> for BounceBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
    );

    fn run(&mut self, (mut ball_storage, transform_storage, paddle_storage): Self::SystemData) {
        for (ball, ball_transform) in (&mut ball_storage, &transform_storage).join() {
            let ball_translation = ball_transform.translation();
            if (ball_translation.y >= ARENA_HEIGHT - ball.radius && ball.velocity.y > 0.0) ||
                (ball_translation.y <= ball.radius && ball.velocity.y < 0.0) {
                ball.velocity.y = -ball.velocity.y;
            }

            for (paddle_transform, paddle) in (&transform_storage, &paddle_storage).join() {
                let paddle_translation = paddle_transform.translation();
                if is_in_rect(
                    &Vector2::new(ball_translation.x, ball_translation.y),
                    &Vector2::new(paddle_translation.x - 0.5 * PADDLE_WIDTH - ball.radius, paddle_translation.y - 0.5 * PADDLE_HEIGHT - ball.radius),
                    &Vector2::new(paddle_translation.x + 0.5 * PADDLE_WIDTH + ball.radius, paddle_translation.y + 0.5 * PADDLE_HEIGHT + ball.radius),
                ) {
                    match paddle.side {
                        Side::Left => {
                            if ball.velocity.x < 0.0 {
                                ball.velocity.x = -ball.velocity.x;
                            }
                        }
                        Side::Right =>
                            if ball.velocity.x > 0.0 {
                                ball.velocity.x = -ball.velocity.x;
                            },
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