use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World,Write, WriteStorage}, ui::UiText,
};

use crate::catvolleyball::{Ball,ARENA_HEIGHT,ARENA_WIDTH,ScoreBoard,ScoreText};

#[derive(SystemDesc)]
pub struct WinnerSystem;
impl<'s> System <'s> for WinnerSystem{
    type SystemData = (WriteStorage<'s, Ball>, 
                WriteStorage<'s, Transform>,
                WriteStorage<'s, UiText>,
                Write<'s, ScoreBoard>,
                ReadExpect<'s, ScoreText>,
            );

    fn run(
        &mut self,
        (mut balls,mut locals, mut ui_text, mut scores,score_text): Self::SystemData,
    ){
        for(ball,transform) in (&mut balls, &mut locals).join(){
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            if ball_y <= ball.radius{
                //touches ground
                if ball_x <= (ARENA_WIDTH*0.5){
                    scores.score_right = (scores.score_right+1).min(999);
                    if let Some(text) = ui_text
                            .get_mut(score_text.p2_score){
                                text.text = scores.score_right.to_string();
                            }
                }else{
                    scores.score_left = (scores.score_left+1).min(999);
                    if let Some(text) = ui_text
                            .get_mut(score_text.p1_score){
                                text.text = scores.score_left.to_string();
                }

                //reset ball to middle
                transform.set_translation_x(ARENA_WIDTH*0.5);
                transform.set_translation_y(ARENA_HEIGHT*0.5);
                //reverse the direction
                ball.velocity[0] = -ball.velocity[0];
                ball.velocity[1] = 0.0;  //reset t0 free drop
            }
        }
    }
}}
