pub mod paleta;

use crate::entidad::Entidad;

struct Lienzo {

    entidades: Vec<Box<Entidad>>,
    contenido: Vec<Vec<u8>>,

}
