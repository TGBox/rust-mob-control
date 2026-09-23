use bevy::prelude::*;
use crate::cannon::Cannon;
use crate::core::constants::CHAMPION_CHARGE_REQUIRED;
use crate::upgrades::GameStats;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct CoinText;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct ChampionBar;

#[derive(Component)]
pub struct ChampionPromptText;

pub fn setup_hud(mut commands: Commands, stats: Res<GameStats>) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Top Bar: Level & Coins
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|top_bar| {
                    // Level text
                    top_bar.spawn((
                        LevelText,
                        Text::new(format!("LEVEL {}", stats.current_level)),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.95, 1.0)),
                    ));

                    // Coins badge
                    top_bar
                        .spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderRadius::all(Val::Px(20.0)),
                            BackgroundColor(Color::srgba(0.1, 0.15, 0.25, 0.85)),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                CoinText,
                                Text::new(format!("💰 {}", stats.coins)),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.85, 0.2)),
                            ));
                        });
                });

            // Bottom Section: Champion Meter & Hint
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|bottom_bar| {
                    // Prompt text
                    bottom_bar.spawn((
                        ChampionPromptText,
                        Text::new("Auto-Firing | Drag / A-D to Aim"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.85, 0.9)),
                    ));

                    // Charge bar background
                    bottom_bar
                        .spawn((
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(16.0),
                                overflow: Overflow::clip(),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(8.0)),
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.3, 0.9)),
                        ))
                        .with_children(|bar_bg| {
                            // Fill bar
                            bar_bg.spawn((
                                ChampionBar,
                                Node {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(8.0)),
                                BackgroundColor(Color::srgb(0.1, 0.6, 1.0)),
                            ));
                        });
                });
        });
}

pub fn update_hud(
    stats: Res<GameStats>,
    cannon_query: Query<&Cannon>,
    mut coin_query: Query<&mut Text, (With<CoinText>, Without<LevelText>, Without<ChampionPromptText>)>,
    mut level_query: Query<&mut Text, (With<LevelText>, Without<CoinText>, Without<ChampionPromptText>)>,
    mut prompt_query: Query<&mut Text, (With<ChampionPromptText>, Without<CoinText>, Without<LevelText>)>,
    mut bar_query: Query<(&mut Node, &mut BackgroundColor), With<ChampionBar>>,
) {
    if let Ok(mut text) = coin_query.get_single_mut() {
        **text = format!("💰 {}", stats.coins);
    }

    if let Ok(mut text) = level_query.get_single_mut() {
        **text = format!("LEVEL {}", stats.current_level);
    }

    if let Ok(cannon) = cannon_query.get_single() {
        let charge_percent = (cannon.champion_charge / CHAMPION_CHARGE_REQUIRED * 100.0).clamp(0.0, 100.0);

        if let Ok((mut node, mut bg)) = bar_query.get_single_mut() {
            node.width = Val::Percent(charge_percent);
            if charge_percent >= 100.0 {
                *bg = BackgroundColor(Color::srgb(1.0, 0.8, 0.1)); // Glowing gold when ready
            } else {
                *bg = BackgroundColor(Color::srgb(0.1, 0.6, 1.0));
            }
        }

        if let Ok(mut text) = prompt_query.get_single_mut() {
            if charge_percent >= 100.0 {
                **text = "⚡ GIANT READY! PRESS SPACE / C ⚡".to_string();
            } else {
                **text = format!("Charging Giant: {}%", charge_percent as i32);
            }
        }
    }
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
