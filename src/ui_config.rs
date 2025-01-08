use bevy::prelude::*;

use crate::tower_types::TowerType;

pub const TOWER_BUTTON_BG_DEFAULT: Color = Color::srgb(0.15, 0.15, 0.15);
pub const TOWER_BUTTON_BG_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
pub const TOWER_BUTTON_BG_SELECTED: Color = Color::srgb(0.05, 0.25, 0.05);

pub const TOWER_BUTTON_BORDER_DEFAULT: Color = Color::BLACK;
pub const TOWER_BUTTON_BORDER_HOVER: Color = Color::WHITE;
pub const TOWER_BUTTON_BORDER_SELECTED: Color = Color::BLACK;

// This controls which tower types are on the buttons
pub const BUTTON_TOWER_TYPES: [TowerType; 4] = [
    TowerType::Freeze,
    TowerType::Minigun,
    TowerType::Piercer,
    TowerType::Sniper,
];
