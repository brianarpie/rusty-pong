use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Read, ReadExpect},
    ecs::prelude::{Join, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use std::ops::Deref;
use crate::audio::{play_score_sound, Sounds};
use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {

    /* We're using Write here to pull in the ScoreBoard instead of with WriteStorage because we want mutable access to ScoreBoard, 
       which is not a collection of components but rather a single resource item. This item is strictly required in all cases, 
       but if we wanted it to be optional we could use Option<Write<'s, ScoreBoard>> instead. */

    /* We also use ReadExpect to access the ScoreText resource immutably. Again, ScoreText is
     * a single resource item rather than a collection of components. With ReadExpect, we are
     * asserting that ScoreText must already exist and will panic if it does not. We do this instead
     * of just using Read because we are manually adding the ScoreText resource to the game in
     * pong.rs > initialise_scoreboard instead of having the system create this resource for us
     * automatically.
     */

    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (
            mut balls, 
            mut locals,
            mut ui_text,
            mut scores,
            score_text,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999); // educational max - not useful
                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0); // reset position
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}


