use amethyst::{
    core::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read,ReadStorage,System, SystemData,World,
        WriteStorage},
    input::{InputHandler,StringBindings},
};

use crate::catvolleyball::{Player,Side,ARENA_WIDTH,PLAYER_WIDTH};

#[derive(SystemDesc)]
pub struct PlayerSystem;
impl<'s>System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );
    fn run(
        &mut self,
        (mut transforms,players,input):Self::SystemData
    ){
        for(player, transform) in
            (&players, &mut transforms).join(){
                let movement = match player.side{
                    Side::Left => input.axis_value("left_player"),
                    Side::Right => input.axis_value("right_player"),
                };
                if let Some(mv_amount) = movement{
                    if mv_amount != 0.0{
                        let side_name = match  {
                            
                        };
                    }
                }
            }
    }
}