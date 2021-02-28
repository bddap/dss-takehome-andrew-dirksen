extern crate alloc;

mod api_types;
mod httpget;
mod image_drawer;
mod uistate;

use crate::uistate::UiState;
use image_drawer::Drawer;
use miniquad::conf::Conf;
use miniquad::conf::{Cache, Loading};
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

/// this would be const if not for the String
fn window_conf() -> Conf {
    Conf {
        cache: Cache::No,
        loading: Loading::No,
        window_title: String::from(""),
        window_width: 1920,
        window_height: 1080,
        high_dpi: false,
        fullscreen: false,
        sample_count: 1,
    }
}

fn main() {
    miniquad::start(window_conf(), |mut ctx| {
        let uistate = crate::uistate::UiState::from_interwebs(&mut ctx).unwrap();
        let stage = Stage {
            uistate,
            imgd: Drawer::new(&mut ctx),
        };
        UserData::owning(stage, ctx)
    });
}
