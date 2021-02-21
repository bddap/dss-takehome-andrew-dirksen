// IFIHADMORETIME
// - these requests would be non-blocking
// - these errors would not be stringly typed
// - mime type would be checked
// - this client would be designed not based on api docs, not sample data
//   - even better than that would be for the creator of the api endpiont to create the client
//   - that client would include its own tests instead of them being included in the unit tests
//     for thismodule
// - fewer allocations
// - strong types for things like ref-id

use crate::api_types;
use core::fmt::Debug;
use reqwest::blocking;
use serde::de::DeserializeOwned;
use std::env::var;
use uuid::Uuid;

pub fn home() -> Result<api_types::Home, String> {
    get_deser("home.json")
}

pub fn get_set(ref_id: &Uuid) -> Result<serde_json::Value, String> {
    get_json(&format!("sets/{}.json", ref_id))
}

pub fn get_jpg(url: &str) -> Result<Img, String> {
    let dat = get_url(url)?;
    Img::decode(&dat)
}

pub struct Img {
    pub width: u16,
    pub height: u16,
    pub rgb: Vec<u8>,
}

impl Img {
    fn decode(bs: &[u8]) -> Result<Self, String> {
        use jpeg_decoder as jpeg;
        let mut decoder = jpeg::Decoder::new(std::io::Cursor::new(bs));
        let pixels = decoder.decode().map_err(dbug)?;
        let metadata = decoder
            .info()
            .expect("metadata should be available after calling decode");
        if metadata.pixel_format != jpeg::PixelFormat::RGB24 {
            Err("unsupported image format")?;
        }
        Ok(Self {
            width: metadata.width,
            height: metadata.height,
            rgb: pixels,
        })
    }
}

fn get_nocache(url: &str) -> Result<Vec<u8>, String> {
    let resp = blocking::get(url).map_err(dbug)?;
    if !resp.status().is_success() {
        return Err(format!(
            "request to {} yeilded a non-success status code.",
            url
        ));
    }
    let bytes = resp.bytes().map_err(dbug).map(|bs| bs.to_vec())?;
    Ok(bytes)
}

fn get_url(url: &str) -> Result<Vec<u8>, String> {
    let cached = cache_dir::get(&url).map_err(dbug)?;
    if let Some(bod) = cached {
        return Ok(bod);
    }
    let ret = get_nocache(&url)?;
    cache_dir::set(&url, &ret).map_err(dbug)?;
    Ok(ret)
}

fn get(path: &str) -> Result<Vec<u8>, String> {
    let mut prefix = var("DSS_API").expect("missing DSS_API enviroment variable");
    if !prefix.ends_with('/') {
        prefix += "/";
    }
    let url = format!("{}{}", prefix, path);
    get_url(&url)
}

fn get_json(path: &str) -> Result<serde_json::Value, String> {
    get(path).and_then(|bs| serde_json::from_slice(&bs).map_err(dbug))
}

fn get_deser<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    get(path).and_then(|bs| serde_json::from_slice(&bs).map_err(dbug))
}

fn dbug(t: impl Debug) -> String {
    format!("{:?}", t)
}

mod cache_dir {
    use sanitize_filename::sanitize;
    use std::fs;
    use std::io;
    use std::io::Read;
    use std::io::Write;

    pub fn get(key: &str) -> io::Result<Option<Vec<u8>>> {
        let mut ret = Vec::new();
        let mut file = match fs::File::open(&format!("cache/{}", sanitize(key))) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };
        file.read_to_end(&mut ret)?;
        Ok(Some(ret))
    }

    pub fn set(key: &str, val: &[u8]) -> io::Result<()> {
        let _ = fs::create_dir("cache");
        fs::File::create(&format!("cache/{}", sanitize(key)))?.write_all(&val)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api_types::Set;

    #[test]
    fn get_all() {
        let home = home().unwrap();
        for set in home
            .data
            .as_sc()
            .containers
            .iter()
            .map(|c| &c.as_shelf_container().set)
            .filter_map(|set| match set {
                Set::CuratedSet { items, .. } => Some(items),
                Set::SetRef { .. } => None,
            })
            .flatten()
        {
            dbg!(&set.image());
        }
    }
}
