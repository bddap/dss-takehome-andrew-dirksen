// IFIHADMORETIME
// - Replace stringly typed error

use crate::api_types::{Item, Set};

const IMAGE_TARGET_ASPECT: f64 = 1.8;

pub struct UiState {
    selected: (usize, usize),
    rows: Vec<(String, Vec<Pick>)>,
}

struct Pick {
    rgba: Vec<[u8; 4]>,
    width: u16,
    height: u16,
}

impl Pick {
    fn blank() -> Self {
        Self {
            rgba: vec![[0, 0, 0, 0]],
            width: 1,
            height: 1,
        }
    }
}

impl UiState {
    pub fn draw(&self, _aspect: f64) {
        for (i, _pick) in self.rows.iter().flat_map(|r| &r.1).enumerate() {
            let d = i as f64 / 2.2342;
            let _center = (d.sin(), d.cos());
            // draw pick at center
        }
    }

    fn get(&self, index: (usize, usize)) -> Option<&Pick> {
        let (i, j) = index;
        self.rows.get(i)?.1.get(j)
    }

    pub fn from_interwebs() -> Result<Self, String> {
        use crate::api_types::*;
        let home: Home = crate::httpget::home()?.data;
        let sc: &StandardCollection = home.as_sc();
        let containers: &[Container] = &sc.containers;
        let containers = containers.iter().map(Container::as_shelf_container);
        let sets = containers.map(|c| &c.set);
        let rows: Vec<(String, Vec<Pick>)> = sets.map(to_row).collect::<Result<_, String>>()?;
        Ok(Self {
            rows,
            selected: (0, 0),
        })
    }
}

fn to_row(set: &crate::api_types::Set) -> Result<(String, Vec<Pick>), String> {
    let title = format!("{:?}", set.text());
    let picks = picks(&deref(set)?.items().unwrap())?;
    Ok((title, picks))
}

fn picks(items: &[Item]) -> Result<Vec<Pick>, String> {
    items.iter().map(Item::image).map(to_pick).collect()
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

fn to_pick(image: &crate::api_types::Image) -> Result<Pick, String> {
    use crate::api_types::*;
    use crate::httpget::{get_jpg, Img};
    let hc: &ImageAspectMap = &image.tile;
    let con: &ImageConcrete = hc
        .get_closest(IMAGE_TARGET_ASPECT)
        .ok_or_else(|| "no image".to_string())?;
    let p = get_jpg(&con.url)
        .map(|Img { width, height, rgb }| Pick {
            width,
            height,
            rgba: rgb.windows(3).map(|wn| [wn[0], wn[1], wn[2], 0]).collect(),
        })
        .unwrap_or_else(|_| Pick::blank());
    Ok(p)
}
