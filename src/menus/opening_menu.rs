use crate::all_states::ActiveState;
use crate::menus::menu_settings;
use ggez::graphics::DrawParam;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, glam, graphics, Context, GameResult};

pub struct MenuState {
    options: Vec<String>,
    selected: usize,
    pub active_state: ActiveState,
}

impl event::EventHandler for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Put your game logic here
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let selected_color = graphics::Color::new(1.0, 0.0, 0.0, 1.0); // Red color for selected
        let unselected_color = graphics::Color::new(1.0, 1.0, 1.0, 1.0); // White color for unselected
        let mut y = 0.0;
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        for (i, option) in self.options.iter().enumerate() {
            let color = if i == self.selected {
                selected_color
            } else {
                unselected_color
            };

            let text = graphics::Text::new(graphics::TextFragment {
                font: Some("LiberationMono-Regular".into()),
                text: option.clone(),
                color: Some(color),
                scale: Some(graphics::PxScale::from(menu_settings::FONT_SIZE)),
                ..Default::default()
            });
            let dest_point = glam::Vec2::new(0.0, y);
            canvas.draw(&text, DrawParam::default().dest(dest_point));
            y += menu_settings::FONT_SIZE;
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    // Other methods...

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match keyinput.keycode {
            Some(KeyCode::Up) => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Some(KeyCode::Down) => {
                if self.selected < self.options.len() - 1 {
                    self.selected += 1;
                }
            }
            Some(KeyCode::Return) => {
                // Handle option selection
                if self.selected == 0 {
                    self.active_state = ActiveState::Game;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl MenuState {
    pub fn new(_ctx: &mut Context) -> MenuState {
        // Initialize your menu state here.
        MenuState {
            options: vec![
                "Start Game".to_string(),
                "Settings".to_string(),
                "Quit".to_string(),
            ],
            selected: 0,
            active_state: ActiveState::Menu,
        }
    }
}
