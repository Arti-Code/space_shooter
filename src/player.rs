use std::sync::Arc;
use bevy::prelude::*;
use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP, BASE_SPEED, PLAYER_LASER_SPRITE, PLAYER_LASER_SIZE};
use crate::components::{Player, Velocity, Movable, SpriteSize, FromPlayer, Laser};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        //.add_system(player_movement_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>, 
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h/2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom+PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpriteSize::from(PLAYER_SIZE))    
    .insert(Player)
    .insert(Movable {auto_despawn: false})
    .insert(Velocity {x: 0.0, y: 0.0});
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>, 
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}

fn player_fire_system(
    mut commands: Commands, 
    kb: Res<Input<KeyCode>>, 
    game_textures: Res<GameTextures>, 
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let mut spawn_laser = |x_offset: f32, y_offset: f32| {
                commands
                .spawn_bundle(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x+x_offset, y-y_offset, 10.0),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Movable {auto_despawn: true})
                .insert(Velocity {x: 0.0, y: 1.0})
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(FromPlayer)
                .insert(Laser);
            };
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;
            let y_offset = (PLAYER_SIZE.1 / 2. - 60.) * SPRITE_SCALE;
            spawn_laser(x_offset, y_offset);
            spawn_laser(-x_offset, y_offset);
        }
    }
}