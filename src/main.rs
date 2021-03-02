// IFIHADMORETIME
// - optimize fps
// - movie modals
// - dynamically populate ref-sets and images
// - bundle ImgDrawer and TxtDrawer into a single Context type.
//   this would allow uistate to concentrate on layout instead of on which system draws which thing
// - cleanup uistate initialization and separate it from layout logic
// - abstract image and text loading from uistate, it shouldn't need to care about generating textures

extern crate alloc;

mod api_types;
mod httpget;
mod image_drawer;
mod text_drawer;
mod uistate;

use crate::uistate::UiState;
use image_drawer::ImgDrawer;
use miniquad::conf::Conf;
use miniquad::conf::{Cache, Loading};
use miniquad::*;
use text_drawer::TxtDrawer;

struct Stage {
    uistate: UiState,
    imgd: ImgDrawer,
    txtd: TxtDrawer,
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        self.uistate.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        self.uistate.draw(ctx, &self.imgd, &self.txtd);
        ctx.commit_frame();
    }

    fn key_down_event(&mut self, ctx: &mut Context, kc: KeyCode, _mods: KeyMods, _repeat: bool) {
        match kc {
            KeyCode::Q | KeyCode::Escape => ctx.request_quit(),
            KeyCode::Left => self.uistate.select_relative((0, -1)),
            KeyCode::Right => self.uistate.select_relative((0, 1)),
            KeyCode::Up => self.uistate.select_relative((-1, 0)),
            KeyCode::Down => self.uistate.select_relative((1, 0)),
            _ => {}
        }
    }
}

/// this would be const if not for the String
fn window_conf() -> Conf {
    Conf {
        cache: Cache::No,
        loading: Loading::No,
        window_title: String::from(""),
        window_width: 1920 / 2,
        window_height: 1080 / 2,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
    }
}

fn main() {
    miniquad::start(window_conf(), |mut ctx| {
        let txtd = TxtDrawer::new(&mut ctx);
        let uistate = crate::uistate::UiState::from_interwebs(&mut ctx, &txtd).unwrap();
        let stage = Stage {
            uistate,
            imgd: ImgDrawer::new(&mut ctx),
            txtd,
        };
        UserData::owning(stage, ctx)
    });
}
