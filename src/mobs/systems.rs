use bevy::prelude::*;
use super::components::*;
use crate::audio::PlaySfx;
use crate::cannon::SpawnMobEvent;
use crate::core::constants::*;
use crate::core::spatial_hash::SpatialHash2D;
use crate::upgrades::{GameStats, Upgrades};
use rand::Rng;

#[derive(Resource)]
pub struct MobAssets {
    pub mob_mesh: Handle<Mesh>,
    pub champion_mesh: Handle<Mesh>,
    pub player_mob_mat: Handle<StandardMaterial>,
    pub player_champ_mat: Handle<StandardMaterial>,
    pub enemy_mob_mat: Handle<StandardMaterial>,
    pub enemy_champ_mat: Handle<StandardMaterial>,
}

pub fn setup_mob_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mob_mesh = meshes.add(Capsule3d::new(MOB_RADIUS, 0.45));
    let champion_mesh = meshes.add(Capsule3d::new(CHAMPION_RADIUS, 1.2));

    let player_mob_mat = materials.add(StandardMaterial {
        base_color: COLOR_PLAYER_MOB,
        emissive: LinearRgba::new(0.05, 0.4, 0.9, 1.0),
        perceptual_roughness: 0.2,
        ..default()
    });

    let player_champ_mat = materials.add(StandardMaterial {
        base_color: COLOR_PLAYER_CHAMPION,
        emissive: LinearRgba::new(0.0, 0.2, 0.8, 1.0),
        metallic: 0.3,
        perceptual_roughness: 0.15,
        ..default()
    });

    let enemy_mob_mat = materials.add(StandardMaterial {
        base_color: COLOR_ENEMY_MOB,
        emissive: LinearRgba::new(0.9, 0.15, 0.15, 1.0),
        perceptual_roughness: 0.2,
        ..default()
    });

    let enemy_champ_mat = materials.add(StandardMaterial {
        base_color: COLOR_ENEMY_CHAMPION,
        emissive: LinearRgba::new(0.8, 0.05, 0.05, 1.0),
        metallic: 0.3,
        perceptual_roughness: 0.15,
        ..default()
    });

    commands.insert_resource(MobAssets {
        mob_mesh,
        champion_mesh,
        player_mob_mat,
        player_champ_mat,
        enemy_mob_mat,
        enemy_champ_mat,
    });
}

pub fn handle_spawn_mob_events(
    mut commands: Commands,
    mut events: EventReader<SpawnMobEvent>,
    mob_assets: Option<Res<MobAssets>>,
    upgrades: Res<Upgrades>,
    mut stats: ResMut<GameStats>,
) {
    let Some(assets) = mob_assets else { return };
    let mut rng = rand::thread_rng();

    for ev in events.read() {
        let (mesh, material, mob) = if ev.is_champion {
            let hp = if ev.is_player {
                upgrades.champion_hp()
            } else {
                CHAMPION_BASE_HP * 1.5
            };
            (
                assets.champion_mesh.clone(),
                if ev.is_player {
                    assets.player_champ_mat.clone()
                } else {
                    assets.enemy_champ_mat.clone()
                },
                Mob::new_champion(ev.is_player, hp, upgrades.mob_speed()),
            )
        } else {
            (
                assets.mob_mesh.clone(),
                if ev.is_player {
                    assets.player_mob_mat.clone()
                } else {
                    assets.enemy_mob_mat.clone()
                },
                if ev.is_player {
                    Mob::new_player(upgrades.mob_speed())
                } else {
                    Mob::new_enemy(MOB_BASE_SPEED * 0.85)
                },
            )
        };

        // Slight jitter on spawn to prevent exact overlap
        let jitter_x = rng.gen_range(-0.15..0.15);
        let jitter_z = rng.gen_range(-0.1..0.1);
        let spawn_pos = ev.position + Vec3::new(jitter_x, 0.0, jitter_z);

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(spawn_pos),
            mob,
            MobAnimation {
                phase: rng.gen_range(0.0..std::f32::consts::TAU),
                base_scale: if ev.is_champion {
                    Vec3::splat(1.3)
                } else {
                    Vec3::ONE
                },
            },
        ));

        if ev.is_player {
            stats.mobs_spawned_total += 1;
        }
    }
}

pub fn update_spatial_hash(
    mut spatial_hash: ResMut<SpatialHash2D>,
    query: Query<(Entity, &Transform, &Mob)>,
) {
    spatial_hash.clear();
    for (entity, transform, _) in query.iter() {
        spatial_hash.insert(entity, Vec2::new(transform.translation.x, transform.translation.z));
    }
}

pub fn update_mob_movement_and_separation(
    time: Res<Time>,
    spatial_hash: Res<SpatialHash2D>,
    mut query: Query<(Entity, &mut Transform, &mut Mob, &mut MobAnimation)>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut mob, mut anim) in query.iter_mut() {
        mob.gate_cooldown.tick(time.delta());

        // Forward motion along Z axis
        let forward_dir = if mob.is_player { -1.0 } else { 1.0 };
        transform.translation.z += forward_dir * mob.speed * dt;

        // Swarm Separation using SpatialHash
        let pos_2d = Vec2::new(transform.translation.x, transform.translation.z);
        let neighbors = spatial_hash.query_radius(pos_2d, MOB_SEPARATION_DISTANCE);

        let mut push = Vec2::ZERO;
        for (other_entity, other_pos, dist_sq) in neighbors {
            if other_entity == entity {
                continue;
            }
            let dist = dist_sq.sqrt();
            if dist > 0.0001 && dist < MOB_SEPARATION_DISTANCE {
                let diff = (pos_2d - other_pos) / dist;
                let strength = (1.0 - (dist / MOB_SEPARATION_DISTANCE)).powf(1.5);
                push += diff * strength;
            }
        }

        // Apply separation push
        transform.translation.x += push.x * MOB_SEPARATION_FORCE * dt;

        // Keep inside runway walls
        let margin = mob.radius + 0.3;
        transform.translation.x = transform.translation.x.clamp(
            -LANE_WIDTH / 2.0 + margin,
            LANE_WIDTH / 2.0 - margin,
        );

        // Animation wobble
        anim.phase += dt * 14.0;
        let bob_y = (anim.phase.sin() * 0.08).abs();
        transform.translation.y = mob.radius + bob_y;

        // Slight squash and stretch
        let stretch = 1.0 + (anim.phase.sin() * 0.08);
        let squash = 1.0 - (anim.phase.sin() * 0.04);
        transform.scale = anim.base_scale * Vec3::new(squash, stretch, squash);
    }
}

pub fn update_mob_combat(
    mut commands: Commands,
    spatial_hash: Res<SpatialHash2D>,
    mut mobs_query: Query<(Entity, &Transform, &mut Mob)>,
    mut stats: ResMut<GameStats>,
    mut sfx: EventWriter<PlaySfx>,
) {
    // 1. Gather clashes (player_entity, enemy_entity)
    let mut clash_pairs = Vec::new();
    let player_mobs: Vec<(Entity, Vec2, f32)> = mobs_query
        .iter()
        .filter(|(_, _, mob)| mob.is_player)
        .map(|(e, tf, mob)| (e, Vec2::new(tf.translation.x, tf.translation.z), mob.radius * 2.0))
        .collect();

    for (p_entity, p_pos, radius) in player_mobs {
        let neighbors = spatial_hash.query_radius(p_pos, radius);
        for (other_entity, _, _) in neighbors {
            if other_entity != p_entity {
                clash_pairs.push((p_entity, other_entity));
                break;
            }
        }
    }

    let mut despawn_list = Vec::new();
    let mut sound_triggered = false;

    // 2. Resolve damage
    for (p_entity, e_entity) in clash_pairs {
        if let Ok([mut player_mob, mut other_mob]) = mobs_query.get_many_mut([p_entity, e_entity]) {
            if player_mob.2.is_player != other_mob.2.is_player {
                player_mob.2.hp -= 1.0;
                other_mob.2.hp -= 1.0;

                if !sound_triggered {
                    sfx.send(PlaySfx::Pop);
                    sound_triggered = true;
                }

                if player_mob.2.hp <= 0.0 {
                    despawn_list.push(p_entity);
                }
                if other_mob.2.hp <= 0.0 {
                    despawn_list.push(e_entity);
                    stats.enemies_defeated_total += 1;
                    stats.coins += 1;
                }
            }
        }
    }

    // Despawn defeated mobs
    for entity in despawn_list {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_mobs(mut commands: Commands, query: Query<Entity, With<Mob>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
