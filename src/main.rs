#![allow(unused_imports, unused)]
mod player;

use bevy::{prelude::*, render::pass::ClearColor, sprite::TextureAtlasBuilder};

use player::PlayerPlugin;


// SPRITES
const PLAYER_IDLE: &str = "Idle.png";
const PLAYER_RUNNING: &str = "Run.png";
const PLAYER_JUMPING: &str = "Jump.png";

//META
const TIME_STEP: f32 = 1./60.;
const GRAVITY: f32 = 1.75;
const GROUND: f32 = 0.0;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Tenagra".to_string(),
            width: 1000.,
            height: 500.,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system().label("setup"))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}

pub struct Materials {
    player_idle: Handle<TextureAtlas>,
    player_running: Handle<TextureAtlas>,
    player_jumping: Handle<TextureAtlas>,
}

struct WinSize {
    w: f32,
    h: f32,
}

struct Player;
struct PlayerState {
    idle: bool,
    running: bool,
    jumping: bool,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            idle: true,
            running: false,
            jumping: false,
        }
    }
}

struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(300.)
    }
}

struct HasJump;
struct JumpReady(bool);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let player_idle_texture_handle = asset_server.load(PLAYER_IDLE); 
    let player_idle_texture_atlas = TextureAtlas::from_grid(player_idle_texture_handle, Vec2::new(128., 64.), 2, 4);
    let player_running_texture_handle = asset_server.load(PLAYER_RUNNING);
    let player_running_texture_atlas = TextureAtlas::from_grid(player_running_texture_handle, Vec2::new(128., 64.), 2, 4);
    let player_jumping_texture_handle = asset_server.load(PLAYER_JUMPING);
    let player_jumping_texture_atlas = TextureAtlas::from_grid(player_jumping_texture_handle, Vec2::new(128., 64.), 2, 4);

    commands.insert_resource(Materials {
        player_idle: texture_atlases.add(player_idle_texture_atlas),
        player_running: texture_atlases.add(player_running_texture_atlas),
        player_jumping: texture_atlases.add(player_jumping_texture_atlas),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });
}

