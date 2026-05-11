#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct AnimaCanvas {
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl AnimaCanvas {
    pub fn new(canvas_id: &str) -> Result<AnimaCanvas, JsValue> {
        let window = web_sys::window().ok_or("No existe la ventana global")?;
        let document = window.document().ok_or("No existe el documento")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("No se encontró el canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or("No se pudo obtener el contexto 2d")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(AnimaCanvas { ctx })
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
