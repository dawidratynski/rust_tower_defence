use bevy::prelude::*;

use crate::enemy::Enemy;

#[derive(Component)]
pub struct CanHaveStatusEffects<T> {
    pub status_effects: Vec<StatusEffect<T>>,
}

impl<T> CanHaveStatusEffects<T> {
    pub fn new() -> CanHaveStatusEffects<T> {
        CanHaveStatusEffects {
            status_effects: vec![],
        }
    }
}

pub struct StatusEffect<T> {
    pub duration: f32,
    pub already_applied: bool,
    pub apply: Box<dyn Fn(&mut T) + Sync + Send>,
    pub tick: Box<dyn Fn(&mut T, f32) + Sync + Send>,
    pub finish: Box<dyn Fn(&mut T) + Sync + Send>,
}

#[derive(Deref)]
pub struct StatusBuilder<T>(pub Box<dyn Fn() -> StatusEffect<T> + Sync + Send>);

pub fn slowdown() -> StatusBuilder<Enemy> {
    StatusBuilder(Box::new(|| StatusEffect {
        duration: 10.0,
        already_applied: false,
        apply: Box::new(|enemy: &mut Enemy| {
            enemy.speed *= 0.8;
        }),
        tick: Box::new(|_, _| {}),
        finish: Box::new(|enemy: &mut Enemy| {
            enemy.speed /= 0.8;
        }),
    }))
}
