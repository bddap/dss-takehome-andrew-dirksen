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
// - dummy sample dataset with tests

use crate::api_types;
use core::fmt::Debug;
use reqwest::blocking;
use serde::de::DeserializeOwned;
use std::env::var;
use uuid::Uuid;

use lazy_static::lazy_static;

lazy_static! {
    static ref API_PREFIX: String = {
        let mut prefix =
            var("DSS_API").unwrap_or("https://cd-static.bamgrid.com/dp-117731241344/".to_string());
        if !prefix.ends_with('/') {
            prefix += "/";
        }
        prefix
    };
}

pub fn home() -> Result<api_types::Wrapped<api_types::Home>, String> {
    get_deser("home.json")
}

pub fn get_set(ref_id: &Uuid) -> Result<api_types::Wrapped<api_types::OuterSet>, String> {
    get_deser(&format!("sets/{}.json", ref_id))
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

pub fn get_url(url: &str) -> Result<Vec<u8>, String> {
    cache_dir::get_or_else(url, || get_nocache(url))
}

fn get(path: &str) -> Result<Vec<u8>, String> {
    let prefix: &str = &API_PREFIX;
    let url = format!("{}{}", prefix, path);
    get_url(&url)
}

fn get_deser<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let bs = get(path)?;
    let deser_result = serde_json::from_slice(&bs);
    deser_result.map_err(dbug)
}

fn dbug(t: impl Debug) -> String {
    format!("{:?}", t)
}

/// this is just to speed development by caching network results
mod cache_dir {
    use lazy_static::lazy_static;
    use sled::Db;

    lazy_static! {
        static ref SLED: Db = sled::open("cache/DB").unwrap();
    }

    pub fn get(url: &str) -> Option<Result<Vec<u8>, String>> {
        let val = SLED.get(url).unwrap()?;
        Some(bincode::deserialize(&val).unwrap())
    }

    pub fn set(url: &str, val: &Result<Vec<u8>, String>) {
        let val = bincode::serialize(val).unwrap();
        SLED.insert(url, val).unwrap();
        SLED.flush().unwrap();
    }

    pub fn get_or_else(
        url: &str,
        f: impl Fn() -> Result<Vec<u8>, String>,
    ) -> Result<Vec<u8>, String> {
        if let Some(ret) = get(url) {
            ret
        } else {
            let ret = f();
            set(url, &ret);
            ret
        }
    }
}
