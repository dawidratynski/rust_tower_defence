use bevy::color::palettes::css;
use bevy::prelude::*;

use crate::bullet::Bullet;
use crate::game_config::TILE_SIZE;
use crate::status_effect::slowdown;
use crate::tower::Tower;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum TowerType {
    Freeze,
    Sniper,
    Minigun,
    Piercer,
}

impl TowerType {
    pub fn get_tower(self) -> (Sprite, Tower) {
        match self {
            TowerType::Freeze => (
                Sprite::from_color(css::LIGHT_BLUE, Vec2::splat(TILE_SIZE) * 0.6),
                Tower {
                    shooting_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: self,
                    spread: 0.0,
                    range: 200.0,
                },
            ),
            TowerType::Sniper => (
                Sprite::from_color(css::DARK_GREEN, Vec2::splat(TILE_SIZE) * 0.6),
                Tower {
                    shooting_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: self,
                    spread: 0.0,
                    range: 50000.0,
                },
            ),
            TowerType::Minigun => (
                Sprite::from_color(css::DARK_MAGENTA, Vec2::splat(TILE_SIZE) * 0.6),
                Tower {
                    shooting_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: self,
                    spread: 0.2,
                    range: 200.0,
                },
            ),
            TowerType::Piercer => (
                Sprite::from_color(css::DARK_RED, Vec2::splat(TILE_SIZE) * 0.6),
                Tower {
                    shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: self,
                    spread: 0.0,
                    range: 400.0,
                },
            ),
        }
    }

    pub fn get_bullet(self, direction: Vec3) -> (Sprite, Bullet) {
        match self {
            TowerType::Freeze => (
                Sprite::from_color(css::LIGHT_BLUE, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 1500.0,
                    hitbox_radius: 30.0,
                    damage: 3.0,
                    pierce: 1,
                    already_hit: vec![],
                    status_effects: vec![slowdown()],
                },
            ),
            TowerType::Sniper => (
                Sprite::from_color(css::DARK_GRAY, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 3000.0,
                    hitbox_radius: 50.0,
                    damage: 50.0,
                    pierce: 1,
                    already_hit: vec![],
                    status_effects: vec![],
                },
            ),
            TowerType::Minigun => (
                Sprite::from_color(css::DARK_GRAY, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 1000.0,
                    hitbox_radius: 20.0,
                    damage: 2.0,
                    pierce: 1,
                    already_hit: vec![],
                    status_effects: vec![],
                },
            ),
            TowerType::Piercer => (
                Sprite::from_color(css::DARK_BLUE, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(10.0, TimerMode::Once),
                    direction,
                    speed: 500.0,
                    hitbox_radius: 50.0,
                    damage: 5.0,
                    pierce: 10,
                    already_hit: vec![],
                    status_effects: vec![],
                },
            ),
        }
    }

    pub fn get_cost(self) -> u32 {
        match self {
            TowerType::Freeze => 15,
            TowerType::Sniper => 30,
            TowerType::Minigun => 50,
            TowerType::Piercer => 20,
        }
    }
}
