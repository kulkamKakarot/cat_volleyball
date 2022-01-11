use amethyst::{core::transform::Transform,
    assets::{AssetStorage,Handle,Loader},
    renderer::{Camera,ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture},
    prelude::*,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode}
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

#[derive(Default)]
pub struct ScoreBoard{
    pub score_left:i32,
    pub score_right:i32,
}

pub struct ScoreText{
    pub p1_score: Entity,
    pub p2_score: Entity,
}
fn intialize_scoreboard(world: &mut World){
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),  // ID
        Anchor::TopMiddle, // anchor
        Anchor::Middle,    // pivot
        -50.,              // x
        -50.,              // y
        1.,                // z
        200.,              // width
        50.,               // height
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::Middle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),  // initial text: 0 points
            [1., 1., 1., 1.], // color (RGBA): white
            50.,              // font size
            LineMode::Single,
            Anchor::Middle
        ))
        .build();
    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle
        ))
        .build();
    world.insert(ScoreText { p1_score, p2_score });
}

pub struct CatVolleyball;
impl SimpleState for CatVolleyball{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        intialize_camera(world);
        intialize_ball(world,sprite_sheet_handle.clone());
        intialize_players(world,sprite_sheet_handle.clone());
        intialize_scoreboard(world);
    }
}
