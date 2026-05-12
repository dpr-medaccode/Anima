use crate::entidad::sprite::Sprite;

pub mod sprite;

pub struct Entidad {

    sprite: Sprite,

    x: i32,
    y: i32,
    
}

impl Entidad {

    fn new(x: i32, y: i32, sprite: Sprite) -> Self {
        Self { sprite, x, y }
    }
    
}
