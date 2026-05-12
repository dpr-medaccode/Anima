pub mod paleta;

use crate::{entidad::Entidad, lienzo::paleta::Pixel};

pub struct Lienzo {

    entidades: Vec<Box<Entidad>>,
    contenido: Vec<Vec<Pixel>>,

}
