use crate::comp;
use crate::res;

use bevy::prelude::*;

/// Plugin bringing in game UI systems and resources
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(health_update_system.system())
            .add_system(energy_update_system.system())
            .add_system(score_update_system.system())
            .add_system(game_over_update_system.system());
    }
}

/// This system updates player's health bar
fn health_update_system(
    mut text_query: Query<(&mut Text, &comp::ui::PlayerHealthDisplay)>,
    mut bar_query: Query<(&mut Style, &comp::ui::PlayerHealthBarDisplay)>,
    mut val_query: Query<(&comp::actor::Player, &comp::stats::HealthStats)>,
) {
    for (mut text, _) in &mut text_query.iter() {
        for (_, health) in &mut val_query.iter() {
            text.value = format!("Hp: {} / {}", health.hull, health.max_hull);
        }
    }
    for (mut style, _) in &mut bar_query.iter() {
        for (_, health) in &mut val_query.iter() {
            let hp_percent = health.hull as f32 / health.max_hull as f32 * 100.0;
            style.size = Size::new(Val::Percent(hp_percent), Val::Percent(4.0));
        }
    }
}

/// This system updates player's energy bar
fn energy_update_system(
    mut text_query: Query<(&mut Text, &comp::ui::PlayerEnergyDisplay)>,
    mut bar_query: Query<(&mut Style, &comp::ui::PlayerEnergyBarDisplay)>,
    mut val_query: Query<(&comp::actor::Player, &comp::stats::EnergyStats)>,
) {
    for (mut text, _) in &mut text_query.iter() {
        for (_, energy) in &mut val_query.iter() {
            text.value = format!("En: {} / {}", energy.energy, energy.max_energy);
        }
    }
    for (mut style, _) in &mut bar_query.iter() {
        for (_, energy) in &mut val_query.iter() {
            let energy_percent = energy.energy as f32 / energy.max_energy as f32 * 100.0;
            style.size = Size::new(Val::Percent(energy_percent), Val::Percent(4.0));
        }
    }
}

/// This system displays the Game Over message
/// on game over.
fn game_over_update_system(
    game_state: Res<res::GameplaySpawnState>,
    mut text_query: Query<(&mut Text, &comp::ui::GameOverDisplay)>,
) {
    if !game_state.game_over {
        return;
    }
    for (mut text, _) in &mut text_query.iter() {
        if text.value == "Game Over" {
            continue;
        }
        text.value = "Game Over".to_string();
    }
}

/// This system updates player's score display
fn score_update_system(
    mut text_query: Query<(&mut Text, &comp::ui::PlayerScoreDisplay)>,
    mut val_query: Query<&comp::actor::Player>,
) {
    for (mut text, _) in &mut text_query.iter() {
        for player in &mut val_query.iter() {
            text.value = format!("Score: {}", player.score);
        }
    }
}
