// IFIHADMORETIME
// - Replace stringly typed error
// - Reuse textures when they are already loaded.

use crate::api_types::{Item, Set};
use crate::httpget::get_url;
use crate::image_drawer::{Drawer, Image, Pos};
use miniquad::Context;

const IMAGE_TARGET_ASPECT: f64 = 1.8;

pub struct UiState {
    selected: (usize, usize),
    rows: Vec<(String, Vec<Pick>)>,
}

struct Pick {
    img: Image,
    pos: Pos,
}

impl Pick {
    fn blank(ctx: &mut Context) -> Self {
        Self {
            img: Image::blank(ctx),
            pos: Pos::default(),
        }
    }
}

impl UiState {
    pub fn update(&mut self) {
        let t = miniquad::date::now();
        let selected = self.selected;
        let numrows = self.rows.len();
        for (i, (_name, row)) in self.rows.iter_mut().enumerate() {
            let rowlen = row.len();
            for (j, pick) in row.iter_mut().enumerate() {
                let target = Self::target_pos(selected, t, (i, j), rowlen, numrows);
                let path = target - pick.pos.clone();
                pick.pos = pick.pos.clone() + path * 0.2;
                // Instantly z to prevent images phasing through eachother.
                pick.pos.z = target.z;
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, imgd: &Drawer) {
        imgd.draw(
            ctx,
            self.rows
                .iter()
                .flat_map(|r| &r.1)
                .map(|pick| (&pick.img, pick.pos.clone())),
        );
    }

    fn target_pos(
        selected: (usize, usize),
        t: f64,
        index: (usize, usize),
        rowlen: usize,
        numrows: usize,
    ) -> Pos {
        if index == selected {
            Pos {
                x: 0.2,
                y: 0.0,
                z: -1.8,
            }
        } else if index.0 == selected.0 {
            // a item in the selected category
            let reord = (selected.1 + rowlen - index.1) % rowlen;
            let angle = map(
                reord as f64,
                0.0,
                rowlen as f64,
                0.0,
                core::f64::consts::TAU,
            ) + t / 50.0;
            Pos {
                x: angle.cos() as f32 * 0.5 + 0.2,
                y: angle.sin() as f32 * 0.5,
                z: map(reord as f64, 0.0, rowlen as f64, -0.5, -0.9) as f32,
            }
        } else {
            // An image from a category that is not selected. most of these will be off-screen and
            // culled.
            let ceord = (selected.0 + numrows + numrows / 4 - index.0) % numrows;
            Pos {
                x: -0.7 - index.1 as f32 / 4.0,
                y: ceord as f32 / 3.0 - 1.0,
                z: -0.8,
            }
        }
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

    pub fn select_relative(&mut self, index: (i8, i8)) {
        self.selected.0 = shift(self.selected.0, index.0 as isize, self.rows.len());
        self.selected.1 = shift(
            self.selected.1,
            index.1 as isize,
            self.rows[self.selected.0].1.len(),
        );
    }
}

/// add shift to current, mod by bound
fn shift(current: usize, shift: isize, bound: usize) -> usize {
    let bound = bound as isize;
    let ret = (current as isize + shift + bound) % bound;
    assert!(ret >= 0);
    ret as usize
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
    Ok(Pick {
        img,
        pos: Pos::default(),
    })
}

/// transforms a number x from range (inmin, inmax) to range (outmin, outmax).
fn map(x: f64, inmin: f64, inmax: f64, outmin: f64, outmax: f64) -> f64 {
    (x - inmin) / (inmax - inmin) * (outmax - outmin) + outmin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmap() {
        assert_eq!(map(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
        assert_eq!(map(0.0, 0.0, 1.0, 0.0, 1.0), 0.0);
        assert_eq!(map(0.5, 0.0, 1.0, 0.0, 1.0), 0.5);
        assert_eq!(map(1.0, 0.0, 2.0, 0.0, 1.0), 0.5);
        assert_eq!(map(0.0, -1.0, 1.0, 0.0, 1.0), 0.5);
        assert_eq!(map(0.0, -1.0, 1.0, -1.0, 0.0), -0.5);
    }
}
