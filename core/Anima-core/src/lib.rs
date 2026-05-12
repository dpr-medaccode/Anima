pub mod lienzo;
pub mod entidad;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Anima {

    ctx: CanvasRenderingContext2d,

}

#[wasm_bindgen]
impl Anima {

  pub fn new_with_element(canvas: HtmlCanvasElement) -> Result<Anima, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .ok_or("No context")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Anima { ctx })
    }

    pub fn draw(&self) {
        self.ctx.set_fill_style_str("blue");
        self.ctx.fill_rect(10.0, 10.0, 100.0, 100.0);

        self.ctx.set_fill_style_str("black");
        self.ctx
            .fill_text("Rust Canvas activo", 10.0, 130.0)
            .unwrap();
    }
}

// https://www.chinedufn.com/3d-webgl-basic-water-tutorial/
// pixels
// webgl
// glow