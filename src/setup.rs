//! Scene and UI setup functionality.
use crate::comp;

use bevy::prelude::*;

/// Plugin that does the game setup and related things
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system())
            .add_startup_system(setup_ui.system());
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server
        .load("assets/fonts/kenvector_future_thin.ttf")
        .unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Score
                    parent
                        .spawn(TextComponents {
                            style: Style {
                                flex_shrink: 0.0,
                                ..Default::default()
                            },
                            text: Text {
                                value: "Score:".to_string(),
                                font: font_handle,
                                style: TextStyle {
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            },
                            ..Default::default()
                        })
                        .with(comp::ui::PlayerScoreDisplay);

                    // Energy
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(4.0)),
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.2).into()),
                            ..Default::default()
                        })
                        .with(comp::ui::PlayerEnergyBarDisplay)
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    style: Style {
                                        flex_shrink: 0.0,
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Energy:".to_string(),
                                        font: font_handle,
                                        style: TextStyle {
                                            font_size: 24.0,
                                            color: Color::rgb(0.0, 0.2, 1.0),
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(comp::ui::PlayerEnergyDisplay);
                        });

                    // Health
                    parent
                        .spawn(NodeComponents {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(4.0)),
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.2).into()),
                            ..Default::default()
                        })
                        .with(comp::ui::PlayerHealthBarDisplay)
                        .with_children(|parent| {
                            parent
                                .spawn(TextComponents {
                                    style: Style {
                                        flex_shrink: 0.0,
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Health:".to_string(),
                                        font: font_handle,
                                        style: TextStyle {
                                            font_size: 24.0,
                                            color: Color::RED,
                                        },
                                    },
                                    ..Default::default()
                                })
                                .with(comp::ui::PlayerHealthDisplay);
                        });
                });

            // Game Over message space
            parent
                .spawn(TextComponents {
                    style: Style {
                        size: Size::new(Val::Percent(80.0), Val::Percent(20.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "".to_string(),
                        font: font_handle,
                        style: TextStyle {
                            font_size: 86.0,
                            color: Color::RED,
                        },
                    },
                    ..Default::default()
                })
                .with(comp::ui::GameOverDisplay);
        });
}

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());
    // player
    spawn_player(&mut commands, &asset_server, &mut materials);
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    // camera
    //let camera = commands.spawn(Camera2dComponents::default()).current_entity().unwrap();

    // player
    let texture_handle = asset_server
        .load("assets/textures/ship/green_player.png")
        .unwrap();
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            translation: Translation::new(0.0, 0.0, 0.2),
            scale: Scale(1.0),
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(comp::physics::Drag(0.45))
        .with(comp::physics::ColliderBox { w: 60, h: 100 })
        .with(comp::actor::Player::default())
        .with(comp::actor::Controller::default())
        .with(comp::stats::Collector)
        .with(comp::stats::MovementSpeed {
            accel: 12.0,
            max: 400.0,
        })
        .with(comp::stats::HealthStats {
            hull: 500,
            max_hull: 500,
        })
        .with(comp::stats::EnergyStats {
            energy: 300,
            max_energy: 300,
        })
        .with(comp::stats::EnergyRegen {
            energy: 1,
            cycle: Timer::from_seconds(0.17),
        })
        .with(comp::stats::Damage { hull: 8 })
        .with(comp::stats::CanDie::default());

    let player = commands.current_entity().unwrap();
    // make player's "ship crash attack" recognized as theirs
    // The kill code really needs a cleanup-revision
    commands.with(comp::stats::DamageOrigin { entity: player });

    // weapons
    let weapon1 = spawn_weapon(
        commands,
        asset_server,
        materials,
        player,
        Translation::new(-20.0, 20.0, 0.3),
    );
    let weapon2 = spawn_weapon(
        commands,
        asset_server,
        materials,
        player,
        Translation::new(20.0, 20.0, 0.3),
    );

    //commands.push_children(player, &[camera]);
    commands.push_children(player, &[weapon1, weapon2]);

    player
}

fn spawn_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    owner: Entity,
    translation: Translation,
) -> Entity {
    let texture_handle = asset_server.load("assets/textures/weapon/gun.png").unwrap();
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            translation,
            scale: Scale(0.7),
            ..Default::default()
        })
        .with(comp::actor::Weapon {
            kind: "nova_blast".to_owned(),
            reload: Timer::from_seconds(0.5),
            energy_drain: 5,
        })
        .with(comp::actor::Controlled { by: owner });

    commands.current_entity().unwrap()
}
