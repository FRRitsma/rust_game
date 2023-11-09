#![allow(clippy::unnecessary_wraps)]

use ggez::graphics::DrawParam;
use ggez::{
    event,
    glam::*,
    graphics::{self, Drawable},
    input::keyboard::KeyInput,
    Context, GameResult,
};
use my_game::collisions::compute_collisions_target_projectile;
use my_game::controls::Controllable;
use my_game::entities::{Lifetime, MovingEntity};
use my_game::menus::opening_menu::MenuState;
use my_game::movement::{BoundaryBehavior, CoordinateMovement, Update};
use my_game::settings;
use my_game::settings::SPRITE_PATH;
use my_game::targets::add_targets;

pub struct GameState {
    main_player: MovingEntity,
    projectile_vec: Vec<MovingEntity>,
    target_vec: Vec<MovingEntity>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        // Define the main player:
        let x_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 800.0, 400.0, 0.0, BoundaryBehavior::Collide);
        let y_axis: CoordinateMovement =
            CoordinateMovement::new(0.0, 600.0, 580.0, 0.0, BoundaryBehavior::Collide);
        let main_player = MovingEntity::new(ctx, x_axis, y_axis);
        // Define a target vector:
        let mut target_vec = Vec::new();
        add_targets(ctx, &mut target_vec);
        Ok(GameState {
            main_player,
            projectile_vec: Vec::new(),
            target_vec,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //Check collisions
        compute_collisions_target_projectile(&mut self.target_vec, &mut self.projectile_vec);
        // Keep only entities that are alive:
        self.target_vec.retain(|entity| entity.is_alive());
        self.projectile_vec.retain(|entity| entity.is_alive());
        // Spawn new targets if existing targets are killed:
        if self.target_vec.is_empty() {
            add_targets(ctx, &mut self.target_vec);
        }
        // Apply updates to entities:
        self.main_player.update();
        self.target_vec.update();
        self.projectile_vec.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        self.main_player.draw(&mut canvas, DrawParam::default());
        for moving_entity in &self.target_vec {
            moving_entity.draw(&mut canvas, DrawParam::default());
        }
        for moving_entity in &self.projectile_vec {
            moving_entity.draw(&mut canvas, DrawParam::default());
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
        // If space bar is lifted, spawn a new entity and attach to entity_vec.
        // If none is returned, add nothing to entity_vec.
        let entity = self.main_player.apply_controllable_up(_ctx, keyinput);
        if let Some(entity) = entity {
            self.projectile_vec.push(entity);
        }
        Ok(())
    }
}

// pub fn main() -> GameResult {
//     let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(
//         ggez::conf::WindowMode::default()
//             .dimensions(settings::WINDOW_WITH, settings::WINDOW_HEIGHT),
//     );
//     let (mut ctx, event_loop) = cb.build()?;
//     ctx.fs.mount(SPRITE_PATH.as_ref(), true);
//     let state = MainState::new(&mut ctx)?;
//     event::run(ctx, event_loop, state);
// }

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(
        ggez::conf::WindowMode::default()
            .dimensions(settings::WINDOW_WITH, settings::WINDOW_HEIGHT),
    );
    let (mut ctx, event_loop) = cb.build()?;
    ctx.fs.mount(SPRITE_PATH.as_ref(), true);
    let state = MenuState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
