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
        let mut y = 0.0;
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        for (i, option) in self.options.iter().enumerate() {
            let text = graphics::Text::new(graphics::TextFragment {
                text: option.clone(),
                ..Default::default()
            });
            let dest_point = glam::Vec2::new(0.0, y);
            canvas.draw(&text, DrawParam::default().dest(dest_point));
            y += 30.0;
            if i == self.selected {
                // Draw a highlight or something
            }
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
