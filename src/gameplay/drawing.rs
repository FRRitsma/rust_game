use crate::gameplay::entities::{Entity, MovingEntity};
use ggez::context::Has;
use ggez::glam;
use ggez::graphics::{Canvas, DrawParam, Drawable, GraphicsContext, Image, Rect};
use ggez::mint::Vector2;

impl Drawable for MovingEntity {
    fn draw(&self, canvas: &mut Canvas, _: impl Into<DrawParam>) {
        let image: &Image = self.get_sprite();

        let my_dest = glam::vec2(self.position().x, self.position().y);
        let scale = Vector2::from_slice(&[
            self.get_sprite_width() / image.width() as f32,
            self.get_sprite_height() / image.height() as f32,
        ]);
        let draw_param = DrawParam::default().dest(my_dest).scale(scale);

        // canvas.draw(image, self.position());
        canvas.draw(image, draw_param);
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
