// Manages all the states of the game, and the transitions between them.

use crate::gameplay::game_state::GameState;
use crate::menus::opening_menu::MenuState;
use ggez::event::EventHandler;
use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameError};

// Derive clone for Active State and partial eq:
#[derive(Clone, PartialEq)]
pub enum ActiveState {
    Menu,
    Game,
}

pub struct AllStates {
    menu_state: Option<MenuState>,
    game_state: Option<GameState>,
    active_state: ActiveState,
}

impl AllStates {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            menu_state: Some(MenuState::new(ctx)),
            game_state: None,
            active_state: ActiveState::Menu,
        }
    }
}

impl EventHandler for AllStates {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.active_state {
            ActiveState::Menu => {
                if let Some(menu_state) = &mut self.menu_state {
                    menu_state.update(_ctx)?;
                    if menu_state.active_state == ActiveState::Game {
                        self.active_state = ActiveState::Game;
                        self.menu_state = None;
                        self.game_state = Some(GameState::new(_ctx).unwrap());
                    }
                }
            }
            ActiveState::Game => {
                if let Some(game_state) = &mut self.game_state {
                    game_state.update(_ctx)?;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.active_state {
            ActiveState::Menu => {
                if let Some(menu_state) = &mut self.menu_state {
                    menu_state.draw(_ctx)?;
                }
            }
            ActiveState::Game => {
                if let Some(game_state) = &mut self.game_state {
                    game_state.draw(_ctx)?;
                }
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        match self.active_state {
            ActiveState::Menu => {
                if let Some(menu_state) = &mut self.menu_state {
                    menu_state.key_down_event(ctx, input, _repeated)?;
                }
            }
            ActiveState::Game => {
                if let Some(game_state) = &mut self.game_state {
                    game_state.key_down_event(ctx, input, _repeated)?;
                }
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        match self.active_state {
            ActiveState::Menu => {
                if let Some(menu_state) = &mut self.menu_state {
                    menu_state.key_up_event(ctx, input)?;
                }
            }
            ActiveState::Game => {
                if let Some(game_state) = &mut self.game_state {
                    game_state.key_up_event(ctx, input)?;
                }
            }
        }
        Ok(())
    }
}
