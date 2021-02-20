// IFIHADMORETIME
// - this client would be designed not based on api docs, not sample data
//   - even better than that would be for the creator of the api endpiont to create the client
// - I don't precicely know the data model of this api so this client will break easily
// - vet external crates more thoroughly

use serde_json::Value;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Home {
    pub data: Data,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Data {
    StandardCollection(StandardCollection),
}

impl Data {
    pub fn as_sc(&self) -> &StandardCollection {
        match &self {
            Self::StandardCollection(sc) => sc,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardCollection {
    pub call_to_action: CallToAction,
    pub collection_group: CollectionGroup,
    pub collection_id: Uuid,
    pub containers: Vec<Container>,
    pub image: StandardCollectionImage,
    pub text: Text,
    pub video_art: Vec<Value>,
    // For some reason this enum is both externally tagged an internally tagged
    // I'm going to ignore the internal tag.
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct StandardCollectionImage {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionGroup {
    pub collection_group_id: Uuid,
    pub content_class: String,
    pub key: String,
    pub slugs: Vec<Slug>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Slug {
    pub language: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Container {
    ShelfContainer(ShelfContainer),
}

impl Container {
    pub fn as_shelf_container(&self) -> &ShelfContainer {
        match &self {
            Self::ShelfContainer(sc) => sc,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShelfContainer {
    pub set: Set,
    pub style: Style,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Style {
    #[serde(rename(serialize = "editorial", deserialize = "editorial"))]
    Editorial,
    BecauseYouSet,
    TrendingSet,
    PersonalizedCuratedSet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Set {
    #[serde(rename_all = "camelCase")]
    CuratedSet {
        content_class: String,
        items: Vec<Item>,
        meta: Meta,
        set_id: Uuid,
        text: Text,
    },
    #[serde(rename_all = "camelCase")]
    SetRef {
        content_class: String,
        ref_id: Uuid,
        ref_id_type: RefIdType,
        ref_type: RefType,
        text: Text,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RefIdType {
    SetId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum RefType {
    BecauseYouSet,
    TrendingSet,
    CuratedSet,
    PersonalizedCuratedSet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Text {
    title: TextTitle,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TextTitle {
    full: TextInner,
    slug: Option<TextInner>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TextInner {
    Series(TextSet),
    Program(TextSet),
    Collection(TextSet),
    Set(TextSet),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TextSet {
    default: TextDefault,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDefault {
    content: Value,
    language: Language,
    source_entity: String,
}

// maybe these fields are floats? maybe sined? deser may fail.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Meta {
    hits: u64,
    offset: i64,
    page_size: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename_all = "camelCase")]
    DmcSeries {
        call_to_action: CallToAction,
        content_id: Uuid,
        current_availability: Value,
        encoded_series_id: Value,
        image: Image,
        media_rights: Value,
        ratings: Value,
        releases: Value,
        series_id: Uuid,
        tags: Value,
        text: Text,
        text_experience_id: Uuid,
        video_art: Value,
    },
    #[serde(rename_all = "camelCase")]
    DmcVideo {
        call_to_action: CallToAction,
        content_id: Uuid,
        content_type: Value,
        current_availability: Value,
        encoded_series_id: Value,
        episode_number: Value,
        episode_sequence_number: Value,
        episode_series_sequence_number: Value,
        family: Value,
        groups: Value,
        image: Image,
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
        text: Text,
        video_art: Value,
        video_id: Value,
    },
    #[serde(rename_all = "camelCase")]
    StandardCollection {
        call_to_action: CallToAction,
        collection_group: Value,
        collection_id: Uuid,
        image: Image,
        text: Text,
        video_art: Value,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    // TODO
}

/// this was always null in sample data
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CallToAction(Value);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Language(String);
