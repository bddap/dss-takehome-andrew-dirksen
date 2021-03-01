use alloc::rc::Rc;
use miniquad::Context;
use miniquad_text_rusttype::TextDisplay;
use miniquad_text_rusttype::{FontAtlas, FontTexture, TextSystem};

pub struct TxtDrawer {
    ts: TextSystem,
    font: Rc<FontTexture>,
}

impl TxtDrawer {
    pub fn new(ctx: &mut Context) -> Self {
        let ts = TextSystem::new(ctx);
        let font = FontTexture::new(
            ctx,
            &include_bytes!("./Roboto-Regular.ttf")[..],
            128,
            FontAtlas::ascii_character_list(),
        )
        .unwrap();
        let font = Rc::new(font);
        Self { ts, font }
    }

    pub fn draw<'a>(
        &self,
        ctx: &mut Context,
        txt: &TextDisplay<Rc<FontTexture>>,
        pos: (f32, f32),
        size: f32,
    ) {
        let (w, h) = ctx.screen_size();
        let aspect = w as f32 / h as f32;
        let matrix = [
            [size, 0.0, 0.0, 0.0],
            [0.0, size * aspect, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [pos.0, pos.1, 0.0, 1.0f32],
        ];
        miniquad_text_rusttype::draw(ctx, txt, &self.ts, matrix, (1.0, 1.0, 1.0, 1.0))
    }

    pub fn create_text_display(
        &self,
        ctx: &mut Context,
        txt: &str,
    ) -> TextDisplay<Rc<FontTexture>> {
        TextDisplay::new(ctx, &self.ts, self.font.clone(), txt)
    }
}
