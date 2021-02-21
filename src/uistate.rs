// IFIHADMORETIME
// - Replace stringly typed error

pub struct UiState {
    selected: (usize, usize),
    rows: Vec<(String, Vec<Pick>)>,
}

struct Pick {
    rgba: Vec<[u8; 4]>,
    width: u16,
    height: u16,
}

impl UiState {
    pub fn draw(&self, aspect: f64) {
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
        let home: Home = crate::httpget::home()?;
        let sc: &StandardCollection = home.data.as_sc();
        let containers: &[Container] = &sc.containers;
        let containers = containers.iter().map(Container::as_shelf_container);
        let sets = containers.map(|c| &c.set);
        let rows: Vec<(String, Vec<Pick>)> = sets.map(to_row).collect::<Result<_, String>>()?;
        Ok(unimplemented!())
    }
}

fn to_row(set: &crate::api_types::Set) -> Result<(String, Vec<Pick>), String> {
    use crate::api_types::{Item, Set};
    let title = match set {
        Set::CuratedSet { text, .. } | Set::SetRef { text, .. } => format!("{:?}", text),
    };
    let picks = match set {
        Set::CuratedSet { items, .. } => items
            .iter()
            .map(Item::image)
            .map(to_pick)
            .collect::<Result<_, _>>()?,
        Set::SetRef { ref_id, .. } => {
            let set = crate::httpget::get_set(ref_id)?;
            unimplemented!()
        }
    };
    Ok((title, picks))
}

fn to_pick(image: &crate::api_types::Image) -> Result<Pick, String> {
    use crate::api_types::*;
    use crate::httpget::{get_jpg, Img};
    dbg!(image);
    let hc: &ImageAspectMap = &image.tile;
    let con: &ImageConcrete = hc.get_closest(1.5).ok_or_else(|| "no image".to_string())?;
    let url: &str = &con.url;
    dbg!(url);
    let Img { width, height, rgb } = get_jpg(url)?;
    Ok(Pick {
        width,
        height,
        rgba: rgb.windows(3).map(|wn| [wn[0], wn[1], wn[2], 0]).collect(),
    })
}
