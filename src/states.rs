use ggez::graphics::DrawParam;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, glam, graphics, Context, GameResult};

pub struct MenuState {
    options: Vec<String>,
    selected: usize,
}

impl event::EventHandler for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Put your game logic here
        Ok(())
    }
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
                    println!("{}", self.selected);
                }
            }
            Some(KeyCode::Down) => {
                if self.selected < self.options.len() - 1 {
                    self.selected += 1;
                    println!("{}", self.selected);
                }
            }
            Some(KeyCode::Return) => {
                // Handle option selection
            }
            _ => {}
        }
        Ok(())
    }
    // Other methods...

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
                text: option.clone(),
                color: Some(color),
                ..Default::default()
            });
            let dest_point = glam::Vec2::new(0.0, y);
            canvas.draw(&text, DrawParam::default().dest(dest_point));
            y += 30.0;
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

impl MenuState {
    pub fn new(_ctx: &mut Context) -> MenuState {
        // Initialize your menu state here.
        MenuState {
            options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                // Add more options here...
            ],
            selected: 0,
        }
    }
}

// pub fn main() -> GameResult {
//     let cb = ggez::ContextBuilder::new("super_simple", "ggez")
//         .window_mode(
//         ggez::conf::WindowMode::default().dimensions(settings::WINDOW_WITH, settings::WINDOW_HEIGHT), // Set window size to 800x600
//     );
//     let (mut ctx, event_loop) = cb.build()?;
//     let state = MainState::new(&mut ctx)?;
//     event::run(ctx, event_loop, state);
// }
