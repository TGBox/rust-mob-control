use bevy::prelude::*;
use crate::core::GameState;
use crate::upgrades::{GameStats, Upgrades};

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub enum MenuButtonAction {
    PlayGame,
    OpenShop,
    NextLevel,
    RetryLevel,
    UpgradeFireRate,
    UpgradeMobSpeed,
    UpgradeBarrels,
    UpgradeChampion,
}

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.08, 0.15, 0.95)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("MOB CONTROL"),
                TextFont {
                    font_size: 46.0,
                    ..default()
                },
                TextColor(Color::srgb(0.1, 0.65, 1.0)),
            ));

            parent.spawn((
                Text::new("Rust ECS Edition"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.8, 0.9)),
            ));

            // Play button
            spawn_button(parent, "START BATTLE", MenuButtonAction::PlayGame, Color::srgb(0.1, 0.65, 0.3));

            // Shop button
            spawn_button(parent, "ARMORY & UPGRADES", MenuButtonAction::OpenShop, Color::srgb(0.1, 0.5, 0.9));
        });
}

pub fn setup_victory_menu(mut commands: Commands, stats: Res<GameStats>) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.1, 0.05, 0.92)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("VICTORY!"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.95, 0.4)),
            ));

            parent.spawn((
                Text::new("Enemy Base Destroyed!"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.95, 0.85)),
            ));

            parent.spawn((
                Text::new(format!("Coins: 💰 {}", stats.coins)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
            ));

            spawn_button(parent, "NEXT LEVEL", MenuButtonAction::NextLevel, Color::srgb(0.1, 0.7, 0.35));
            spawn_button(parent, "UPGRADE SHOP", MenuButtonAction::OpenShop, Color::srgb(0.1, 0.5, 0.9));
        });
}

pub fn setup_game_over_menu(mut commands: Commands, stats: Res<GameStats>) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.02, 0.04, 0.92)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("DEFEAT!"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.25)),
            ));

            parent.spawn((
                Text::new("Cannon Breached!"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.85, 0.85)),
            ));

            parent.spawn((
                Text::new(format!("Coins: 💰 {}", stats.coins)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
            ));

            spawn_button(parent, "RETRY LEVEL", MenuButtonAction::RetryLevel, Color::srgb(0.85, 0.25, 0.25));
            spawn_button(parent, "UPGRADE SHOP", MenuButtonAction::OpenShop, Color::srgb(0.1, 0.5, 0.9));
        });
}

pub fn setup_shop_menu(mut commands: Commands, stats: Res<GameStats>, upgrades: Res<Upgrades>) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.07, 0.14, 0.97)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("ARMORY UPGRADES"),
                TextFont {
                    font_size: 34.0,
                    ..default()
                },
                TextColor(Color::srgb(0.15, 0.7, 1.0)),
            ));

            parent.spawn((
                Text::new(format!("Available Coins: 💰 {}", stats.coins)),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
            ));

            // Upgrade 1: Fire Rate
            let fr_label = format!(
                "⚡ Fire Rate (Lvl {}) - Cost: 💰 {}",
                upgrades.fire_rate_level,
                upgrades.fire_rate_cost()
            );
            spawn_button(parent, &fr_label, MenuButtonAction::UpgradeFireRate, Color::srgb(0.15, 0.45, 0.85));

            // Upgrade 2: Mob Speed
            let speed_label = format!(
                "🏃 Mob Speed (Lvl {}) - Cost: 💰 {}",
                upgrades.mob_speed_level,
                upgrades.mob_speed_cost()
            );
            spawn_button(parent, &speed_label, MenuButtonAction::UpgradeMobSpeed, Color::srgb(0.2, 0.5, 0.85));

            // Upgrade 3: Barrels
            let barrel_label = if upgrades.cannon_barrels < 3 {
                format!(
                    "💥 Multi-Barrel ({}) - Cost: 💰 {}",
                    if upgrades.cannon_barrels == 1 { "Dual" } else { "Triple" },
                    upgrades.barrel_cost()
                )
            } else {
                "💥 Max Barrels (Triple)".to_string()
            };
            spawn_button(parent, &barrel_label, MenuButtonAction::UpgradeBarrels, Color::srgb(0.75, 0.4, 0.1));

            // Upgrade 4: Champion Power
            let champ_label = format!(
                "🛡️ Giant Power (Lvl {}) - Cost: 💰 {}",
                upgrades.champion_level,
                upgrades.champion_cost()
            );
            spawn_button(parent, &champ_label, MenuButtonAction::UpgradeChampion, Color::srgb(0.5, 0.2, 0.75));

            // Back button
            spawn_button(parent, "BACK TO BATTLE", MenuButtonAction::PlayGame, Color::srgb(0.1, 0.65, 0.3));
        });
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, action: MenuButtonAction, bg_color: Color) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(280.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                margin: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(bg_color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text),
                TextFont {
                    font_size: 17.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut stats: ResMut<GameStats>,
    mut upgrades: ResMut<Upgrades>,
) {
    for (interaction, action, _) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::PlayGame => {
                    next_state.set(GameState::Playing);
                }
                MenuButtonAction::OpenShop => {
                    next_state.set(GameState::Shop);
                }
                MenuButtonAction::NextLevel => {
                    next_state.set(GameState::Playing);
                }
                MenuButtonAction::RetryLevel => {
                    next_state.set(GameState::Playing);
                }
                MenuButtonAction::UpgradeFireRate => {
                    let cost = upgrades.fire_rate_cost();
                    if stats.coins >= cost {
                        stats.coins -= cost;
                        upgrades.fire_rate_level += 1;
                        next_state.set(GameState::Shop);
                    }
                }
                MenuButtonAction::UpgradeMobSpeed => {
                    let cost = upgrades.mob_speed_cost();
                    if stats.coins >= cost {
                        stats.coins -= cost;
                        upgrades.mob_speed_level += 1;
                        next_state.set(GameState::Shop);
                    }
                }
                MenuButtonAction::UpgradeBarrels => {
                    if upgrades.cannon_barrels < 3 {
                        let cost = upgrades.barrel_cost();
                        if stats.coins >= cost {
                            stats.coins -= cost;
                            upgrades.cannon_barrels += 1;
                            next_state.set(GameState::Shop);
                        }
                    }
                }
                MenuButtonAction::UpgradeChampion => {
                    let cost = upgrades.champion_cost();
                    if stats.coins >= cost {
                        stats.coins -= cost;
                        upgrades.champion_level += 1;
                        next_state.set(GameState::Shop);
                    }
                }
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
