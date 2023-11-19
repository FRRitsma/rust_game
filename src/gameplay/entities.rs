use crate::gameplay::movement::{Position, Update};
use ggez;

use ggez::graphics::Image;

use crate::gameplay::gameplay_settings::{
    ENEMY_SCALE_HEIGHT, ENEMY_SCALE_WIDTH, ENEMY_SPRITE, MAIN_PLAYER_SCALE_HEIGHT,
    MAIN_PLAYER_SCALE_WIDTH, MAIN_PLAYER_SPRITE, PROJECTILE_SCALE_HEIGHT, PROJECTILE_SCALE_WIDTH,
    PROJECTILE_SPRITE,
};
use crate::gameplay::movement::CoordinateMovement;
use ggez::{glam, Context};
use glam::Vec2;

pub enum EntityType {
    Player,
    Enemy,
    Projectile,
}

pub trait Entity {
    fn position(&self) -> Vec2;
}

pub trait Lifetime {
    fn is_alive(&self) -> bool;
    fn set_alive(&mut self, alive: bool);
}

pub struct MovingEntity {
    pub x_axis: CoordinateMovement,
    pub y_axis: CoordinateMovement,
    sprite: Option<Image>,
    sprite_width: Option<f32>,
    sprite_height: Option<f32>,
    is_alive: bool,
}

impl Lifetime for MovingEntity {
    fn is_alive(&self) -> bool {
        // Both axes and the object itself must still be alive:
        self.x_axis.is_alive() && self.y_axis.is_alive() && self.is_alive
    }
    fn set_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }
}

impl Entity for MovingEntity {
    fn position(&self) -> Vec2 {
        Vec2::new(self.x_axis.get_position(), self.y_axis.get_position())
    }
}

impl MovingEntity {
    pub fn new(x_axis: CoordinateMovement, y_axis: CoordinateMovement) -> Self {
        MovingEntity {
            x_axis,
            y_axis,
            sprite: None,
            sprite_width: None,
            sprite_height: None,
            is_alive: true,
        }
    }
    pub fn get_sprite(&self) -> &Image {
        self.sprite.as_ref().unwrap()
    }

    pub fn get_sprite_width(&self) -> f32 {
        self.sprite_width.unwrap()
    }
    pub fn get_sprite_height(&self) -> f32 {
        self.sprite_height.unwrap()
    }
    pub fn set_entity_type(&mut self, entity_type: EntityType, ctx: &mut Context) {
        match entity_type {
            EntityType::Player => self.set_player(ctx),
            EntityType::Enemy => self.set_enemy(ctx),
            EntityType::Projectile => self.set_projectile(ctx),
        }
    }

    fn set_player(&mut self, ctx: &mut Context) {
        self.sprite = Some(Image::from_path(ctx, MAIN_PLAYER_SPRITE).unwrap());
        self.sprite_width = Some(MAIN_PLAYER_SCALE_WIDTH);
        self.sprite_height = Some(MAIN_PLAYER_SCALE_HEIGHT);
    }
    fn set_enemy(&mut self, ctx: &mut Context) {
        self.sprite = Some(Image::from_path(ctx, ENEMY_SPRITE).unwrap());
        self.sprite_width = Some(ENEMY_SCALE_WIDTH);
        self.sprite_height = Some(ENEMY_SCALE_HEIGHT);
    }
    fn set_projectile(&mut self, ctx: &mut Context) {
        self.sprite = Some(Image::from_path(ctx, PROJECTILE_SPRITE).unwrap());
        self.sprite_width = Some(PROJECTILE_SCALE_WIDTH);
        self.sprite_height = Some(PROJECTILE_SCALE_HEIGHT);
    }
}

impl Update for MovingEntity {
    fn update(&mut self) {
        self.x_axis.update();
        self.y_axis.update();
    }
}

impl Update for Vec<MovingEntity> {
    fn update(&mut self) {
        for moving_entity in self.iter_mut() {
            moving_entity.update();
        }
    }
}
