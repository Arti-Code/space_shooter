use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::{GameTextures, SPRITE_SCALE, ENEMY_SIZE, WinSize, EnemyCount, ENEMY_MAX};
use crate::components::{Enemy, SpriteSize};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system),
        );
    }
}


fn enemy_spawn_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>, 
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. -100.;
        let h_span = win_size.h / 2. -100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));
        enemy_count.0 += 1;
    }
}