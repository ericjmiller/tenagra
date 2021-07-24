use bevy::prelude::*;

use crate::{
    Materials, Player, PlayerState, Speed, HasJump, JumpReady, WinSize,
    GRAVITY, GROUND, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_stage(
                "player_setup",
                SystemStage::single(spawn_idle_player.system())
            )
            .add_system(player_movement.system().label("player_movement"))
            .add_system(player_state_change.system().label("player_state_change").after("player_movement"))
            .add_system(player_animation.system().label("player_animation").after("player_state_change"))
            .add_system(gravity.system().label("gravity").after("player_state_change"))
            .run();
    }
}

fn spawn_idle_player(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
){
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: materials.player_idle.clone(),
        ..Default::default()
    })
    .insert(Player)
    .insert(Speed::default())
    .insert(PlayerState::default())
    .insert(Timer::from_seconds(0.10, true))
    .insert(HasJump)
    .insert(JumpReady(true));
}

fn player_animation(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &PlayerState, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Player>>,
) {
    if let Ok((entity, player_state, mut timer, mut sprite, texture_atlas_handle)) = query.single_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index += 1;
            if sprite.index == texture_atlas.textures.len() as u32 {
                sprite.index = 0;
            }
        }
    } 
}

fn player_jumping(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, &mut PlayerState, &mut TextureAtlasSprite), With<Player>>,
) {
    if let Ok((speed, mut transform, mut player_state, mut sprite)) = query.single_mut() {
        
    }
}
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, &mut PlayerState, &mut TextureAtlasSprite, &mut JumpReady), With<Player>>,
) {
    if let Ok((speed, mut transform, mut player_state, mut sprite, mut jump_ready)) = query.single_mut() {
        let mut dir_x: f32 = 0.;
        let mut dir_y: f32 = 0.;
        player_state.idle = true;
        player_state.running = false;
        player_state.jumping = false;

        if keyboard_input.pressed(KeyCode::A) {
            player_state.idle = false;
            player_state.running = true;
            sprite.flip_x = true;
            dir_x = -1.;
        } 

        if keyboard_input.pressed(KeyCode::D) {
            player_state.idle = false;
            player_state.running = true;
            sprite.flip_x = false;
            dir_x = 1.;
        } 

        //TODO: Need better jump functionality
        if jump_ready.0 && keyboard_input.pressed(KeyCode::Space) {
            player_state.jumping = true;
            dir_y = 50.;
            jump_ready.0 = false;
        }

        if keyboard_input.just_released(KeyCode::Space) {
            jump_ready.0 = true;
        }

        transform.translation.x += dir_x * speed.0 * TIME_STEP;
        transform.translation.y += dir_y;
    } 
}

fn player_state_change(
    texture_atlases: Res<Assets<TextureAtlas>>,
    materials: Res<Materials>,
    mut query: Query<(&PlayerState, &mut Handle<TextureAtlas>), With<Player>>,
){
    if let Ok((player_state, mut texture_atlas_handle)) = query.single_mut() {
        if player_state.running && !player_state.idle {
            *texture_atlas_handle = materials.player_running.clone();
        } else if !player_state.running && player_state.idle {
            *texture_atlas_handle = materials.player_idle.clone();
        } else if player_state.jumping {
            *texture_atlas_handle = materials.player_jumping.clone();
        }
    }
}

fn gravity(
    mut query: Query<(&mut Transform)>,
) {
    for (mut transform) in query.iter_mut() {
        if transform.translation.y > GROUND {
            transform.translation.y -= GRAVITY;
        }
    }
}

