use crate::gameplay::collisions::compute_collisions_target_projectile;
use crate::gameplay::controls::Controllable;
use crate::gameplay::enemy_wave::EnemyWave;
use crate::gameplay::entities::{EntityType, Lifetime, MovingEntity};
use crate::gameplay::movement::{BoundaryBehavior, CoordinateMovement, Update};

use ggez::glam::bool;
use ggez::graphics::{DrawParam, Drawable};
use ggez::input::keyboard::KeyInput;
use ggez::{event, graphics, Context, GameResult};

pub struct GameState {
    main_player: MovingEntity,
    projectile_vec: Vec<MovingEntity>,
    enemy_wave: EnemyWave,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        // Define the main player:
        let x_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 800.0, 400.0, 0.0, BoundaryBehavior::Collide);
        let y_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 600.0, 580.0, 0.0, BoundaryBehavior::Collide);
        let mut main_player = MovingEntity::new(x_axis, y_axis);
        main_player.set_entity_type(EntityType::Player, ctx);
        // Define a target vector:
        Ok(GameState {
            main_player,
            projectile_vec: Vec::new(),
            enemy_wave: EnemyWave::new(),
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Check collisions
        compute_collisions_target_projectile(
            self.enemy_wave.get_enemies(),
            &mut self.projectile_vec,
        );
        // Keep only entities that are alive:
        self.projectile_vec.retain(|entity| entity.is_alive());
        // Spawn new targets if existing targets are killed, after a certain delay
        self.enemy_wave.update(ctx);
        // Apply updates to entities:
        self.main_player.update();
        self.projectile_vec.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.main_player.draw(&mut canvas, DrawParam::default());
        for moving_entity in &self.projectile_vec {
            moving_entity.draw(&mut canvas, DrawParam::default());
        }
        for enemy in self.enemy_wave.get_enemies() {
            enemy.draw(&mut canvas, DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        self.main_player.apply_controllable_down(keyinput);
        Ok(())
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keyinput: KeyInput) -> GameResult {
        let entity = self.main_player.apply_controllable_up(_ctx, keyinput);
        if let Some(entity) = entity {
            self.projectile_vec.push(entity);
        }
        Ok(())
    }
}
