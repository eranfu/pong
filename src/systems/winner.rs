use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    ecs::{Join, Read, ReadExpect, System, Write, WriteStorage},
    ui::UiText,
};

use crate::audio::{play_score_sound, Sounds};
use crate::pong::{ARENA_WIDTH, Ball, ScoreBoard, ScoreText};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        ReadExpect<'s, Sounds>,
        Read<'s, AssetStorage<Source>>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (mut ball_storage, mut transform_storage, mut text_storage, mut score_board, score_text, sounds, source_storage, output): Self::SystemData) {
        for (ball, transform) in (&mut ball_storage, &mut transform_storage).join() {
            let did_hit = {
                let ball_x = transform.translation().x;
                if ball_x <= ball.radius {
                    score_board.score_right = (score_board.score_right + 1).min(999);
                    if let Some(text) = text_storage.get_mut(score_text.p2_score) {
                        text.text = score_board.score_right.to_string();
                    }
                    true
                } else if ball_x >= ARENA_WIDTH - ball.radius {
                    score_board.score_left = (score_board.score_left + 1).min(999);
                    if let Some(text) = text_storage.get_mut(score_text.p1_score) {
                        text.text = score_board.score_left.to_string();
                    }
                    true
                } else {
                    false
                }
            };

            if did_hit {
                ball.velocity.x = -ball.velocity.x;
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                play_score_sound(&sounds, &source_storage, output.as_ref().map(|o| o.deref()));

                println!("Score: | {} | {} |", score_board.score_left, score_board.score_right);
            }
        }
    }
}