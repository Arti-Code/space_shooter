#![allow(unused)]

mod player;
mod components;
mod enemy;

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use components::{Movable, Velocity};
use player::PlayerPlugin;
use enemy::EnemyPlugin;
//use player::player_spawn_system;

// region:  ---CONSTANTS
const PLAYER_SPRITE: &str = "shooter.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const PLAYER_SIZE: (f32, f32) = (128., 128.);
const ENEMY_SIZE: (f32, f32) = (93., 84.);
const SPRITE_SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);
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
}
// endregion: ---RESOURCES

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "Space Shooter".to_string(),
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
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    window.set_position(IVec2::new(1000, 100));

    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE)
    };
    commands.insert_resource(game_textures);
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

fn set_window_icon(windows: NonSend<WinitWindows>,) {
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