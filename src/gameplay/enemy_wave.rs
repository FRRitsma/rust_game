use crate::gameplay::entities::MovingEntity;
use crate::gameplay::entities::{EntityType, Lifetime};
use crate::gameplay::gameplay_settings::{
    ENEMIES_PER_ROW, ENEMY_SPACING, ENEMY_SPEED, RESPAWN_TIME,
};
use crate::gameplay::movement::Update;
use crate::gameplay::movement::{BoundaryBehavior, CoordinateMovement};
use crate::settings::WINDOW_HEIGHT;

use ggez::Context;
use std::time::Instant;

pub struct EnemyWave {
    timer: Option<Instant>,
    enemies: Vec<MovingEntity>,
}

impl EnemyWave {
    pub fn new() -> EnemyWave {
        EnemyWave {
            timer: None,
            enemies: Vec::new(),
        }
    }

    pub fn get_enemies(&mut self) -> &mut Vec<MovingEntity> {
        &mut self.enemies
    }

    fn add_targets(&mut self, ctx: &mut Context) {
        // Test if timer is none. If timer is none, instantiate timer:
        if let Some(initialized_timer) = &mut self.timer {
            if initialized_timer.elapsed().as_secs() < RESPAWN_TIME {
                return;
            }
            self.timer = None;
            let mut x_axis: CoordinateMovement;
            let y_axis: CoordinateMovement = CoordinateMovement::new(
                0.0,
                WINDOW_HEIGHT,
                ENEMY_SPACING,
                0.25,
                BoundaryBehavior::Bounce,
            );
            for i in 0..ENEMIES_PER_ROW {
                x_axis = CoordinateMovement::new(
                    50.0 + ENEMY_SPACING * i as f32,
                    350.0 + ENEMY_SPACING * i as f32,
                    ENEMY_SPACING * i as f32,
                    ENEMY_SPEED,
                    BoundaryBehavior::Bounce,
                );
                let mut target = MovingEntity::new(x_axis, y_axis);
                target.set_entity_type(EntityType::Enemy, ctx);
                self.enemies.push(target);
            }
        } else {
            self.timer = Some(Instant::now());
        }
    }
    pub fn update(&mut self, ctx: &mut Context) {
        for enemy in self.enemies.iter_mut() {
            enemy.update();
        }
        self.enemies.retain(|entity| entity.is_alive());
        if self.enemies.is_empty() {
            self.add_targets(ctx);
        }
    }
}
