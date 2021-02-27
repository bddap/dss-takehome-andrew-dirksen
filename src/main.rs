extern crate alloc;

mod api_types;
mod httpget;
mod image_drawer;
mod uistate;

use crate::uistate::UiState;
use image_drawer::Drawer;
use miniquad::*;

struct Stage {
    uistate: UiState,
    imgd: Drawer,
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        self.uistate.draw(ctx, &self.imgd);
        ctx.commit_frame();
    }

    fn key_down_event(&mut self, ctx: &mut Context, kc: KeyCode, _mods: KeyMods, _repeat: bool) {
        if KeyCode::Q == kc {
            ctx.request_quit();
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        let uistate = crate::uistate::UiState::from_interwebs(&mut ctx).unwrap();
        let stage = Stage {
            uistate,
            imgd: Drawer::new(&mut ctx),
        };
        UserData::owning(stage, ctx)
    });
}
