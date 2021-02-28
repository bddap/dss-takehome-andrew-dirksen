// IFIHADMORETIME
// - Replace stringly typed error

use crate::api_types::{Item, Set};
use crate::httpget::get_url;
use crate::image_drawer::{Drawer, Image, Pos, Scale};
use miniquad::Context;

const IMAGE_TARGET_ASPECT: f64 = 1.8;

pub struct UiState {
    selected: (usize, usize),
    rows: Vec<(String, Vec<Pick>)>,
}

struct Pick {
    img: Image,
}

impl Pick {
    fn blank(ctx: &mut Context) -> Self {
        Self {
            img: Image::from_rgba8(ctx, 1, 1, &[0, 0, 0, 0]),
        }
    }
}

impl UiState {
    pub fn draw(&self, ctx: &mut Context, imgd: &Drawer) {
        let t = miniquad::date::now();
        for (i, pick) in self.rows.iter().flat_map(|r| &r.1).enumerate() {
            let d = i as f64 / 10.3579 + t / 4.0;
            let center = Pos {
                x: (d.sin() / 2.0) as f32,
                y: ((d * 0.9).cos() / 2.0) as f32,
            };
            // draw pick at center
            imgd.draw_single(ctx, &pick.img, center, Scale { big: false });
        }
        if let Some(pick) = self.get_selected() {
            imgd.draw_single(ctx, &pick.img, Pos { x: 0.0, y: 0.0 }, Scale { big: true });
        }
    }

    fn get(&self, index: (usize, usize)) -> Option<&Pick> {
        let (i, j) = index;
        self.rows.get(i)?.1.get(j)
    }

    fn get_selected(&self) -> Option<&Pick> {
        self.get(self.selected)
    }

    pub fn from_interwebs(ctx: &mut Context) -> Result<Self, String> {
        use crate::api_types::*;
        let home: Home = crate::httpget::home()?.data;
        let sc: &StandardCollection = home.as_sc();
        let containers: &[Container] = &sc.containers;
        let containers = containers.iter().map(Container::as_shelf_container);
        let sets = containers.map(|c| &c.set);
        let rows: Vec<(String, Vec<Pick>)> = sets
            .map(|set| to_row(ctx, set))
            .collect::<Result<_, String>>()?;
        Ok(Self {
            rows,
            selected: (0, 0),
        })
    }
}

fn to_row(ctx: &mut Context, set: &crate::api_types::Set) -> Result<(String, Vec<Pick>), String> {
    let title = format!("{:?}", set.text());
    let picks = picks(ctx, &deref(set)?.items().unwrap())?;
    Ok((title, picks))
}

fn picks(ctx: &mut Context, items: &[Item]) -> Result<Vec<Pick>, String> {
    items
        .iter()
        .map(Item::image)
        .map(|img| to_pick(ctx, img))
        .collect()
}

/// to api request for set if needed.
fn deref(set: &Set) -> Result<Set, String> {
    let ret = match set {
        Set::SetRef(sr) => {
            let wrapped = crate::httpget::get_set(&sr.ref_id)?;
            wrapped.data.inner().clone()
        }
        a => a.clone(),
    };
    if let Set::SetRef { .. } = ret {
        return Err("reference to reference".to_string());
    }
    Ok(ret)
}

fn to_pick(ctx: &mut Context, image: &crate::api_types::Image) -> Result<Pick, String> {
    use crate::api_types::ImageAspectMap;
    use crate::api_types::ImageConcrete;
    let hc: &ImageAspectMap = &image.tile;
    let con: &ImageConcrete = hc
        .get_closest(IMAGE_TARGET_ASPECT)
        .ok_or_else(|| "no image".to_string())?;

    let req = get_url(&con.url);
    let bs: &[u8] = match &req {
        Ok(bs) => bs,
        Err(_) => return Ok(Pick::blank(ctx)),
    };
    let img = Image::decode_jpg(ctx, bs)?;
    Ok(Pick { img })
}
