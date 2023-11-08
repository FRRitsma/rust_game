use crate::entities::{Entity, MovingEntity};
use ggez::context::Has;
use ggez::graphics::{Canvas, DrawParam, Drawable, GraphicsContext, Rect};

impl Drawable for MovingEntity {
    fn draw(&self, canvas: &mut Canvas, _: impl Into<DrawParam>) {
        canvas.draw(self.get_sprite(), self.position());
    }
    fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        let image = self.get_sprite();
        Some(Rect::new(
            self.position().x,
            self.position().y,
            image.width() as f32,
            image.height() as f32,
        ))
    }
}
