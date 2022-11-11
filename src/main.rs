#![allow(unused)]

mod player;
mod components;
mod enemy;

use bevy::math::Vec3Swizzles;
use bevy::utils::HashSet;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use components::{Movable, Velocity, FromPlayer, SpriteSize, Laser, Enemy, Explosion, ExplosionTimer, ExplosionToSpawn};
use player::PlayerPlugin;
use enemy::EnemyPlugin;
//use player::player_spawn_system;

// region:  ---CONSTANTS
const PLAYER_SPRITE: &str = "shooter.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "alien.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const PLAYER_SIZE: (f32, f32) = (128., 128.);
const ENEMY_SIZE: (f32, f32) = (96., 96.);
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const ENEMY_MAX: u32 = 2;
const EXPLOSION_LEN: usize = 16;
const MARGIN: f32 = 200.;
// endregion    ---CONSTANTS

// region: ---RESOURCES
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}
// endregion: ---RESOURCES

struct EnemyCount(u32);

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "Space Shooter v0.3.1".to_string(),
        width: 598.0,
        height: 576.0,
        ..Default::default()
    };
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(window_descriptor)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(set_window_icon)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    window.set_position(IVec2::new(25, 25));
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
}

fn movable_system(
    mut commands: Commands, 
    win_size: Res<WinSize>, 
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        if movable.auto_despawn {
            if translation.y > win_size.h / 2. + MARGIN 
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.w / 2. + MARGIN
                || translation.x < -win_size.w / 2. - MARGIN
            {
                println!("---> despawn {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("shooter.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity) ||
                despawned_entities.contains(&laser_entity) {
                    continue;
            }
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            let collision = collide(
                laser_tf.translation,
                laser_size.0*laser_scale,
                enemy_tf.translation,
                enemy_size.0*laser_scale,
            );
            
            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);
                commands.spawn().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(explosion_spawn_entity).despawn();
            
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}

