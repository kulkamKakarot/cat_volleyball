use amethyst::{core::transform::Transform,
    assets::{AssetStorage,Handle,Loader},
    renderer::{Camera,ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture},
    prelude::*,
    ecs::prelude::{Component, DenseVecStorage}
};

pub const ARENA_HEIGHT:f32 = 500.0;
pub const ARENA_WIDTH:f32 = 500.0;
pub const PLAYER_HEIGHT:f32 = 64.0;
pub const PLAYER_WIDTH:f32 = 64.0;

pub const BALL_VELOCITY_X:f32 = 30.0;
pub const BALL_VELOCITY_Y:f32 = 0.0;
pub const BALL_RADIUS:f32 = 4.0;
pub struct Ball{
    pub velocity:[f32; 2],
    pub radius: f32,
}

impl Component for Ball{
    type Storage = DenseVecStorage<Self>;
}


pub enum Side{
    Left,
    Right,
}

pub struct Player{
    pub side: Side,
    pub width: f32,
    pub height: f32,
}
impl Player{
    fn new(side: Side) -> Player{
        Player{
            side,
            width:PLAYER_WIDTH,
            height:PLAYER_HEIGHT,
        }
    }
}
impl Component for Player{
    type Storage = DenseVecStorage<Self>;
}

fn intialize_players(world: &mut World,sprite_sheet: Handle<SpriteSheet>){
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let sprite_render = SpriteRender{
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    let y = PLAYER_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PLAYER_WIDTH*0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH-PLAYER_WIDTH*0.5,y,0.0);
    right_transform.set_rotation_y_axis(std::f32::consts::PI);
    world.create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Left))
        .with(left_transform)
        .build();
    world.create_entity()
        .with(sprite_render.clone())
        .with(Player::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet>{
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world
            .read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world
            .read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn intialize_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        ARENA_WIDTH*0.5,
        ARENA_HEIGHT*0.5,
        1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


///Intializes one ball the middle of arena
fn intialize_ball(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>
){
    //create translation
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH*0.5, ARENA_HEIGHT*0.5,0.0);
    //assign the sprite for ball
    let sprite_render = SpriteRender{
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };
    world.create_entity()
        .with(sprite_render)
        .with(Ball{
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X,BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

pub struct CatVolleyball;
impl SimpleState for CatVolleyball{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        intialize_camera(world);
        world.register::<Ball>();
        intialize_ball(world,sprite_sheet_handle.clone());
        intialize_players(world,sprite_sheet_handle.clone());
    }
}
