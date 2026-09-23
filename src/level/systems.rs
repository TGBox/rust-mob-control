use bevy::prelude::*;
use super::components::*;
use super::levels::get_level_definition;
use crate::audio::PlaySfx;
use crate::cannon::SpawnMobEvent;
use crate::core::constants::*;
use crate::core::GameState;
use crate::gates::spawn_gate_entity;
use crate::mobs::Mob;
use crate::upgrades::GameStats;
use rand::Rng;

pub fn setup_global_camera_and_lighting(mut commands: Commands) {
    // Persistent 3D Camera for both 3D scene and Bevy UI overlay
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 16.5, 30.5).looking_at(Vec3::new(0.0, 0.0, 3.0), Vec3::Y),
    ));

    // Directional Sun Light
    commands.spawn((
        DirectionalLight {
            illuminance: 14000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light for soft filling
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.95, 1.0),
        brightness: 450.0,
    });
}

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    stats: Res<GameStats>,
) {
    let level_def = get_level_definition(stats.current_level);

    // 1. Runway Floor
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(LANE_WIDTH, LANE_LENGTH));
    let floor_mat = materials.add(StandardMaterial {
        base_color: COLOR_FLOOR,
        perceptual_roughness: 0.4,
        ..default()
    });

    commands.spawn((
        LevelEnvironment,
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Center lane dashed markings
    let stripe_mesh = meshes.add(Plane3d::default().mesh().size(0.28, 2.2));
    let stripe_mat = materials.add(StandardMaterial {
        base_color: COLOR_FLOOR_ACCENT,
        perceptual_roughness: 0.5,
        ..default()
    });

    for i in -5..=5 {
        let z = i as f32 * 4.0;
        commands.spawn((
            LevelEnvironment,
            Mesh3d(stripe_mesh.clone()),
            MeshMaterial3d(stripe_mat.clone()),
            Transform::from_xyz(0.0, 0.01, z),
        ));
    }

    // 2. Side Walls / Rails
    let wall_mesh = meshes.add(Cuboid::new(0.6, 0.9, LANE_LENGTH));
    let wall_mat = materials.add(StandardMaterial {
        base_color: COLOR_WALL,
        metallic: 0.6,
        perceptual_roughness: 0.3,
        ..default()
    });

    // Left wall
    commands.spawn((
        LevelEnvironment,
        Mesh3d(wall_mesh.clone()),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(-LANE_WIDTH / 2.0 - 0.3, 0.45, 0.0),
    ));

    // Right wall
    commands.spawn((
        LevelEnvironment,
        Mesh3d(wall_mesh),
        MeshMaterial3d(wall_mat),
        Transform::from_xyz(LANE_WIDTH / 2.0 + 0.3, 0.45, 0.0),
    ));

    // 3. Enemy Base
    let base_mesh = meshes.add(Cuboid::new(LANE_WIDTH * 0.85, 3.5, 2.5));
    let base_mat = materials.add(StandardMaterial {
        base_color: COLOR_BASE,
        emissive: LinearRgba::new(0.6, 0.05, 0.05, 1.0),
        perceptual_roughness: 0.3,
        ..default()
    });

    commands
        .spawn((
            LevelEnvironment,
            EnemyBase {
                current_hp: level_def.base_hp,
                max_hp: level_def.base_hp,
            },
            Mesh3d(base_mesh),
            MeshMaterial3d(base_mat),
            Transform::from_xyz(0.0, 1.75, BASE_Z),
        ))
        .with_children(|parent| {
            // HP Bar Text
            parent.spawn((
                BaseHpText,
                Text2d::new(format!("{}/{}", level_def.base_hp as i32, level_def.base_hp as i32)),
                TextFont {
                    font_size: 52.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 2.8, 1.35)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_6 * 0.5))
                    .with_scale(Vec3::splat(0.03)),
            ));
        });

    // 4. Enemy Spawner
    commands.spawn((
        LevelEnvironment,
        EnemySpawner {
            spawn_timer: Timer::from_seconds(level_def.spawn_interval, TimerMode::Repeating),
            champion_timer: Timer::from_seconds(8.0, TimerMode::Repeating),
            mobs_per_wave: level_def.mobs_per_wave,
            waves_remaining: 1000,
        },
    ));

    // 5. Spawn Multiplier Gates
    for gate_cfg in level_def.gates {
        spawn_gate_entity(
            &mut commands,
            &mut meshes,
            &mut materials,
            gate_cfg.gate_type,
            gate_cfg.movement,
            gate_cfg.width,
            gate_cfg.position,
        );
    }
}

pub fn update_enemy_spawner(
    time: Res<Time>,
    mut spawner_query: Query<&mut EnemySpawner>,
    stats: Res<GameStats>,
    mut spawn_events: EventWriter<SpawnMobEvent>,
) {
    let Ok(mut spawner) = spawner_query.get_single_mut() else {
        return;
    };
    let mut rng = rand::thread_rng();

    spawner.spawn_timer.tick(time.delta());
    if spawner.spawn_timer.just_finished() {
        for _ in 0..spawner.mobs_per_wave {
            let offset_x = rng.gen_range(-LANE_WIDTH * 0.35..LANE_WIDTH * 0.35);
            spawn_events.send(SpawnMobEvent {
                position: Vec3::new(offset_x, 0.1, BASE_Z + 2.0),
                direction: Vec3::new(0.0, 0.0, 1.0),
                is_player: false,
                is_champion: false,
            });
        }
    }

    // Champions spawn in level >= 4
    if stats.current_level >= 4 {
        spawner.champion_timer.tick(time.delta());
        if spawner.champion_timer.just_finished() {
            let offset_x = rng.gen_range(-LANE_WIDTH * 0.25..LANE_WIDTH * 0.25);
            spawn_events.send(SpawnMobEvent {
                position: Vec3::new(offset_x, 0.3, BASE_Z + 2.5),
                direction: Vec3::new(0.0, 0.0, 1.0),
                is_player: false,
                is_champion: true,
            });
        }
    }
}

pub fn update_base_combat_and_win(
    mut commands: Commands,
    mut base_query: Query<(&mut EnemyBase, Entity)>,
    mut hp_text_query: Query<&mut Text2d, With<BaseHpText>>,
    player_mobs_query: Query<(Entity, &Transform, &Mob)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut stats: ResMut<GameStats>,
    mut sfx: EventWriter<PlaySfx>,
    mut shake: ResMut<crate::juice::CameraShake>,
) {
    let Ok((mut base, _)) = base_query.get_single_mut() else {
        return;
    };

    let mut hit_base = false;

    for (mob_entity, transform, mob) in player_mobs_query.iter() {
        if mob.is_player && transform.translation.z <= BASE_Z + 1.2 {
            let damage = if mob.is_champion { 6.0 } else { 1.0 };
            base.current_hp = (base.current_hp - damage).max(0.0);
            commands.entity(mob_entity).despawn();
            stats.coins += 2;
            hit_base = true;
        }
    }

    if hit_base {
        shake.trigger(0.25);
        if let Ok(mut text) = hp_text_query.get_single_mut() {
            **text = format!("{}/{}", base.current_hp as i32, base.max_hp as i32);
        }
    }

    // Win condition!
    if base.current_hp <= 0.0 {
        stats.coins += 50 * stats.current_level as u32; // Level completion bonus
        stats.current_level += 1;
        shake.trigger(0.6);
        sfx.send(PlaySfx::Victory);
        next_state.set(GameState::LevelWon);
    }
}

pub fn update_defeat_condition(
    enemy_mobs_query: Query<&Transform, With<Mob>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sfx: EventWriter<PlaySfx>,
) {
    for transform in enemy_mobs_query.iter() {
        if transform.translation.z >= CANNON_Z - 0.5 {
            // Breach!
            sfx.send(PlaySfx::Defeat);
            next_state.set(GameState::GameOver);
            break;
        }
    }
}

pub fn cleanup_level(mut commands: Commands, query: Query<Entity, With<LevelEnvironment>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
