// IFIHADMORETIME
// - this client would be designed not based on api docs, not sample data
//   - even better than that would be for the creator of the api endpiont to create the client
// - I don't precicely know the data model of this api so this client will break easily
// - vet external crates more thoroughly

use serde_json::Value;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Home {
    pub data: Data,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Data {
    #[serde(rename_all = "camelCase")]
    StandardCollection {
        call_to_action: Value,
        collection_group: Value,
        collection_id: Uuid,
        containers: Vec<Container>,
        image: Value,
        text: Value,
        video_art: Value,
        // For some reason this type is both externally tagged an internally tagged
        // I'm going to ignore the internal "type" tag.
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Container {
    #[serde(rename_all = "camelCase")]
    ShelfContainer { set: Set, style: String },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Set {
    #[serde(rename_all = "camelCase")]
    CuratedSet {
        content_class: String,
        items: Vec<Item>,
        meta: Value,
        set_id: Uuid,
        text: Value,
    },
    #[serde(rename_all = "camelCase")]
    SetRef {
        content_class: String,
        ref_id: Uuid,
        ref_id_type: Value,
        ref_type: Value,
        text: Value,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename_all = "camelCase")]
    DmcSeries {
        call_to_action: Value,
        content_id: Uuid,
        current_availability: Value,
        encoded_series_id: Value,
        image: Value,
        media_rights: Value,
        ratings: Value,
        releases: Value,
        series_id: Uuid,
        tags: Value,
        text: Value,
        text_experience_id: Uuid,
        video_art: Value,
    },
    #[serde(rename_all = "camelCase")]
    DmcVideo {
        call_to_action: Value,
        content_id: Uuid,
        content_type: Value,
        current_availability: Value,
        encoded_series_id: Value,
        episode_number: Value,
        episode_sequence_number: Value,
        episode_series_sequence_number: Value,
        family: Value,
        groups: Value,
        image: Value,
        internal_title: Value,
        media_metadata: Value,
        media_rights: Value,
        original_language: Value,
        program_id: Uuid,
        program_type: Value,
        ratings: Value,
        releases: Value,
        season_id: Value,
        season_sequence_number: Value,
        series_id: Value,
        tags: Value,
        target_language: Value,
        text: Value,
        video_art: Value,
        video_id: Value,
    },
    #[serde(rename_all = "camelCase")]
    StandardCollection {
        call_to_action: Value,
        collection_group: Value,
        collection_id: Uuid,
        image: Value,
        text: Value,
        video_art: Value,
    },
}
