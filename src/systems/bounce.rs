use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::catvolleyball::{Ball, Player, Side, ARENA_HEIGHT, ARENA_WIDTH};

use super::PlayerSystem;

#[derive(SystemDesc)]
pub struct BounceSystem;
impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, players, transforms): Self::SystemData) {
        /* checking ball bouncing
        and ball velocity every time
        to prevent multiple collisions */
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            //Bounce at four sides of arena
            if ball_y <= ball.radius && ball.velocity[1] < 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            } else if ball_y >= (ARENA_HEIGHT - ball.radius) && ball.velocity[1] > 0.0 {
                ball.velocity[1] = -ball.velocity[1];
            } else if ball_x <= (ball.radius) && ball.velocity[0] < 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            } else if ball_x >= (ARENA_WIDTH - ball.radius) && ball.velocity[0] > 0.0 {
                ball.velocity[0] = -ball.velocity[0];
            }
        }
    }
}
