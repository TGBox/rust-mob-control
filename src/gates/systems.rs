use bevy::prelude::*;
use super::components::*;
use crate::audio::PlaySfx;
use crate::cannon::SpawnMobEvent;
use crate::core::constants::*;
use crate::mobs::Mob;
use rand::Rng;

pub fn spawn_gate_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    gate_type: GateType,
    movement: GateMovement,
    width: f32,
    pos: Vec3,
) -> Entity {
    let (bg_color, emissive) = if gate_type.is_buff() {
        if matches!(gate_type, GateType::Multiply(v) if v >= 4) {
            (COLOR_GATE_GOLD, LinearRgba::new(0.9, 0.7, 0.05, 0.8))
        } else {
            (COLOR_GATE_BLUE, LinearRgba::new(0.05, 0.6, 1.0, 0.8))
        }
    } else {
        (COLOR_GATE_RED, LinearRgba::new(0.9, 0.1, 0.1, 0.8))
    };

    let panel_mat = materials.add(StandardMaterial {
        base_color: bg_color.with_alpha(0.65),
        emissive,
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.1,
        ..default()
    });

    let frame_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.25, 0.35),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    let panel_mesh = meshes.add(Cuboid::new(width, 1.8, 0.15));
    let post_mesh = meshes.add(Cylinder::new(0.18, 2.2));
    let label = gate_type.label();

    commands
        .spawn((
            Gate {
                gate_type,
                width,
                movement,
            },
            GateVisual,
            Transform::from_translation(pos),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Energy Glass Panel
            parent.spawn((
                Mesh3d(panel_mesh),
                MeshMaterial3d(panel_mat),
                Transform::from_xyz(0.0, 0.9, 0.0),
            ));

            // Left Post
            parent.spawn((
                Mesh3d(post_mesh.clone()),
                MeshMaterial3d(frame_mat.clone()),
                Transform::from_xyz(-width / 2.0, 1.1, 0.0),
            ));

            // Right Post
            parent.spawn((
                Mesh3d(post_mesh),
                MeshMaterial3d(frame_mat),
                Transform::from_xyz(width / 2.0, 1.1, 0.0),
            ));

            // 3D Gate Text Label
            parent.spawn((
                Text2d::new(label),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 1.0, 0.12)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_6 * 0.3))
                    .with_scale(Vec3::splat(0.025)),
            ));
        })
        .id()
}

pub fn update_gate_movement(
    time: Res<Time>,
    mut query: Query<(&mut Gate, &mut Transform)>,
) {
    let dt = time.delta_secs();

    for (mut gate, mut transform) in query.iter_mut() {
        if let GateMovement::Sine {
            amplitude,
            speed,
            center_x,
            ref mut phase,
        } = gate.movement
        {
            *phase += speed * dt;
            let offset = phase.sin() * amplitude;
            transform.translation.x = center_x + offset;
        }
    }
}

pub fn update_gate_mob_collisions(
    mut mob_query: Query<(Entity, &Transform, &mut Mob)>,
    gate_query: Query<(Entity, &Gate, &Transform)>,
    mut spawn_events: EventWriter<SpawnMobEvent>,
    mut sfx: EventWriter<PlaySfx>,
) {
    let mut rng = rand::thread_rng();

    for (_mob_entity, mob_tf, mut mob) in mob_query.iter_mut() {
        if !mob.is_player || !mob.gate_cooldown.finished() {
            continue;
        }

        for (gate_entity, gate, gate_tf) in gate_query.iter() {
            // Check if mob is in the gate slice
            let dx = (mob_tf.translation.x - gate_tf.translation.x).abs();
            let dz = (mob_tf.translation.z - gate_tf.translation.z).abs();

            if dx <= (gate.width / 2.0) && dz <= 0.6 {
                if mob.last_gate_entity == Some(gate_entity) {
                    continue;
                }

                // Passed gate!
                mob.last_gate_entity = Some(gate_entity);
                mob.gate_cooldown.reset();

                sfx.send(PlaySfx::GateChime);

                let copies_to_spawn = match gate.gate_type {
                    GateType::Multiply(mult) => {
                        if mult > 1 {
                            // If champion passes, spawn extra champions or standard mobs
                            if mob.is_champion {
                                (mult - 1).min(3)
                            } else {
                                mult - 1
                            }
                        } else {
                            0
                        }
                    }
                    GateType::Add(count) => {
                        if mob.is_champion {
                            1
                        } else {
                            count
                        }
                    }
                    GateType::Sub(_) => 0,
                    GateType::Divide(_) => 0,
                };

                for i in 0..copies_to_spawn {
                    let spread_x = rng.gen_range(-0.4..0.4);
                    let spread_z = (i as f32 * 0.25) + rng.gen_range(-0.1..0.1);
                    spawn_events.send(SpawnMobEvent {
                        position: mob_tf.translation + Vec3::new(spread_x, 0.0, spread_z),
                        direction: Vec3::new(0.0, 0.0, -1.0),
                        is_player: true,
                        is_champion: mob.is_champion,
                    });
                }
                break;
            }
        }
    }
}

pub fn cleanup_gates(mut commands: Commands, query: Query<Entity, With<Gate>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
