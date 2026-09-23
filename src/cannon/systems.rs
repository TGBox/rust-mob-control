use bevy::prelude::*;
use super::components::*;
use crate::audio::PlaySfx;
use crate::core::constants::*;
use crate::upgrades::Upgrades;

#[derive(Component)]
pub struct CannonRoot;

pub fn setup_cannon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    upgrades: Res<Upgrades>,
) {
    let base_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.2, 0.3),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });

    let barrel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.6, 1.0),
        metallic: 0.5,
        perceptual_roughness: 0.2,
        ..default()
    });

    let body_mesh = meshes.add(Cuboid::new(1.8, 0.8, 1.4));
    let barrel_mesh = meshes.add(Cylinder::new(0.28, 1.2));

    commands
        .spawn((
            CannonRoot,
            Cannon {
                fire_timer: Timer::from_seconds(1.0 / upgrades.shots_per_second(), TimerMode::Repeating),
                champion_charge: 0.0,
                recoil: 0.0,
            },
            Transform::from_xyz(0.0, 0.5, CANNON_Z),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Main body block
            parent.spawn((
                Mesh3d(body_mesh),
                MeshMaterial3d(base_material),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            // Barrels based on upgrade
            match upgrades.cannon_barrels {
                2 => {
                    parent.spawn((
                        CannonBarrel { offset_x: -0.45 },
                        Mesh3d(barrel_mesh.clone()),
                        MeshMaterial3d(barrel_material.clone()),
                        Transform::from_xyz(-0.45, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                    parent.spawn((
                        CannonBarrel { offset_x: 0.45 },
                        Mesh3d(barrel_mesh),
                        MeshMaterial3d(barrel_material),
                        Transform::from_xyz(0.45, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                }
                3 => {
                    parent.spawn((
                        CannonBarrel { offset_x: -0.6 },
                        Mesh3d(barrel_mesh.clone()),
                        MeshMaterial3d(barrel_material.clone()),
                        Transform::from_xyz(-0.6, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                    parent.spawn((
                        CannonBarrel { offset_x: 0.0 },
                        Mesh3d(barrel_mesh.clone()),
                        MeshMaterial3d(barrel_material.clone()),
                        Transform::from_xyz(0.0, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                    parent.spawn((
                        CannonBarrel { offset_x: 0.6 },
                        Mesh3d(barrel_mesh),
                        MeshMaterial3d(barrel_material),
                        Transform::from_xyz(0.6, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                }
                _ => {
                    // Single barrel
                    parent.spawn((
                        CannonBarrel { offset_x: 0.0 },
                        Mesh3d(barrel_mesh),
                        MeshMaterial3d(barrel_material),
                        Transform::from_xyz(0.0, 0.2, -0.6).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ));
                }
            }
        });
}

pub fn update_cannon_movement(
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cannon_query: Query<&mut Transform, With<CannonRoot>>,
) {
    let Ok(mut transform) = cannon_query.get_single_mut() else {
        return;
    };

    let mut target_x = transform.translation.x;

    // Mouse drag / cursor position
    if let Ok(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            // Check if mouse left button is held or auto-tracking
            if mouse_button.pressed(MouseButton::Left) {
                let norm_x = (cursor_pos.x / window.width() - 0.5) * 2.0; // -1.0 to 1.0
                target_x = norm_x * (LANE_WIDTH * 0.45);
            }
        }
    }

    // Keyboard fallback
    let move_speed = 18.0;
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        target_x -= move_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        target_x += move_speed * time.delta_secs();
    }

    target_x = target_x.clamp(CANNON_MIN_X, CANNON_MAX_X);
    // Smooth lerp
    transform.translation.x = transform.translation.x.lerp(target_x, 20.0 * time.delta_secs());
}

pub fn update_cannon_shooting(
    time: Res<Time>,
    upgrades: Res<Upgrades>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cannon_query: Query<(&mut Cannon, &Transform)>,
    barrel_query: Query<&CannonBarrel>,
    mut spawn_events: EventWriter<SpawnMobEvent>,
    mut sfx_events: EventWriter<PlaySfx>,
    mut shake: ResMut<crate::juice::CameraShake>,
) {
    let Ok((mut cannon, transform)) = cannon_query.get_single_mut() else {
        return;
    };

    // Auto-fire or fire while holding left click / space
    let should_fire = mouse_button.pressed(MouseButton::Left)
        || keys.pressed(KeyCode::Space)
        || keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::ArrowUp)
        || true; // Continuous autofire for quintessential hyper-casual Mob Control feel!

    // Update fire rate timer from upgrades
    let target_interval = 1.0 / upgrades.shots_per_second();
    if (cannon.fire_timer.duration().as_secs_f32() - target_interval).abs() > 0.001 {
        cannon.fire_timer.set_duration(std::time::Duration::from_secs_f32(target_interval));
    }

    cannon.fire_timer.tick(time.delta());

    if should_fire && cannon.fire_timer.just_finished() {
        // Spawn mob from each barrel
        for barrel in barrel_query.iter() {
            let spawn_pos = transform.translation + Vec3::new(barrel.offset_x, 0.1, -1.2);
            spawn_events.send(SpawnMobEvent {
                position: spawn_pos,
                direction: Vec3::new(0.0, 0.0, -1.0),
                is_player: true,
                is_champion: false,
            });
        }

        // Charge champion slightly with each shot
        cannon.champion_charge = (cannon.champion_charge + 1.0).min(CHAMPION_CHARGE_REQUIRED);
        cannon.recoil = 0.2;

        sfx_events.send(PlaySfx::Shoot);
    }

    // Trigger Champion summon if charged and button pressed
    if cannon.champion_charge >= CHAMPION_CHARGE_REQUIRED
        && (keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::KeyC))
    {
        cannon.champion_charge = 0.0;
        spawn_events.send(SpawnMobEvent {
            position: transform.translation + Vec3::new(0.0, 0.3, -2.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
            is_player: true,
            is_champion: true,
        });
        shake.trigger(0.45);
        sfx_events.send(PlaySfx::ChampionStomp);
    }

    // Recoil recovery
    cannon.recoil = (cannon.recoil - 2.0 * time.delta_secs()).max(0.0);
}

pub fn cleanup_cannon(mut commands: Commands, query: Query<Entity, With<CannonRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
