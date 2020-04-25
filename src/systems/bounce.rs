use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Read, ReadExpect},
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::pong::{Ball, Side, Paddle, ARENA_HEIGHT};
use crate::audio::{play_bounce_sound, Sounds};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
      );


    fn run(&mut self, (
            mut balls, 
            paddles, 
            transforms,
            storage,
            sounds,
            audio_output
        ): Self::SystemData) {
        // check whether ball collided, apply transform accordingly

        // also, check velocity of ball to prevent multiple collisions
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            
            if (ball_y <= ball.radius && ball.velocity[1] < 0.0)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0)
            {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

               // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                    ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                        {
                            ball.velocity[0] = -ball.velocity[0];
                            play_bounce_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                        }
                }

                // A point is in a box when its coordinates are smaller or equal than the top
                // right and larger or equal than the bottom left.
                fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
                    x >= left && x <= right && y >= bottom && y <= top
                }

            }

        }
    }

}
