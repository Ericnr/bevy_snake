use bevy::prelude::*;

/// Marker for displaying player's health
#[derive(Debug, Default, Properties)]
pub struct PlayerHealthDisplay;

/// Marker for displaying player's health
#[derive(Debug, Default, Properties)]
pub struct PlayerHealthBarDisplay;

/// Marker for displaying player's energy
#[derive(Debug, Default, Properties)]
pub struct PlayerEnergyDisplay;

/// Marker for displaying player's energy
#[derive(Debug, Default, Properties)]
pub struct PlayerEnergyBarDisplay;

/// Marker for displaying player's score
#[derive(Debug, Default, Properties)]
pub struct PlayerScoreDisplay;

/// Marker for the Game Over display
#[derive(Debug, Default, Properties)]
pub struct GameOverDisplay;
