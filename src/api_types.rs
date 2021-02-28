// IFIHADMORETIME
// - this client would be designed not based on api docs, not sample data
//   - even better than that would be for the creator of the api endpiont to create the client
// - I don't precicely know the data model of this api so this client will break easily
// - vet external crates more thoroughly
// - come up with a nice abstraction for "things that are either T, or a ref_id of T"

use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Wrapped<T> {
    pub data: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Home {
    StandardCollection(StandardCollection),
}

impl Home {
    pub fn as_sc(&self) -> &StandardCollection {
        match &self {
            Self::StandardCollection(sc) => sc,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum OuterSet {
    CuratedSet(Set),
    TrendingSet(Set),
    SetRef(Set),
    PersonalizedCuratedSet(Set),
}

impl OuterSet {
    pub fn inner(&self) -> &Set {
        match &self {
            Self::CuratedSet(set)
            | Self::TrendingSet(set)
            | Self::SetRef(set)
            | Self::PersonalizedCuratedSet(set) => set,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct StandardCollectionImage {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionGroup {
    pub collection_group_id: Uuid,
    pub content_class: String,
    pub key: String,
    pub slugs: Vec<Slug>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Slug {
    pub language: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShelfContainer {
    pub set: Set,
    pub style: Style,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Style {
    #[serde(rename(serialize = "editorial", deserialize = "editorial"))]
    Editorial,
    BecauseYouSet,
    TrendingSet,
    PersonalizedCuratedSet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Set {
    CuratedSet(CuratedSet),
    TrendingSet(TrendingSet),
    SetRef(SetRef),
    PersonalizedCuratedSet(PersonalizedCuratedSet),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersonalizedCuratedSet {
    content_class: String,
    pub items: Vec<Item>,
    meta: Meta,
    set_id: Uuid,
    pub text: Text,
    experiment_token: Value,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CuratedSet {
    content_class: String,
    pub items: Vec<Item>,
    meta: Meta,
    set_id: Uuid,
    pub text: Text,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrendingSet {
    content_class: String,
    items: Vec<Item>,
    meta: Meta,
    set_id: Uuid,
    text: Text,
    experiment_token: Value,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetRef {
    content_class: String,
    pub ref_id: Uuid,
    ref_id_type: RefIdType,
    ref_type: RefType,
    text: Text,
}

impl Set {
    pub fn text(&self) -> &Text {
        match &self {
            Self::CuratedSet(CuratedSet { text, .. })
            | Self::TrendingSet(TrendingSet { text, .. })
            | Self::SetRef(SetRef { text, .. })
            | Self::PersonalizedCuratedSet(PersonalizedCuratedSet { text, .. }) => text,
        }
    }

    pub fn items(&self) -> Option<&[Item]> {
        match &self {
            Self::CuratedSet(CuratedSet { items, .. })
            | Self::TrendingSet(TrendingSet { items, .. })
            | Self::PersonalizedCuratedSet(PersonalizedCuratedSet { items, .. }) => Some(items),
            Self::SetRef { .. } => None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RefIdType {
    SetId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum RefType {
    BecauseYouSet,
    TrendingSet,
    CuratedSet,
    PersonalizedCuratedSet,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Text {
    title: TextTitle,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TextTitle {
    full: TextInner,
    slug: Option<TextInner>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TextInner {
    Series(TextSet),
    Program(TextSet),
    Collection(TextSet),
    Set(TextSet),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextSet {
    default: TextDefault,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextDefault {
    content: Value,
    language: Language,
    source_entity: String,
}

// maybe these fields are floats? maybe sined? deser may fail.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meta {
    hits: u64,
    offset: i64,
    page_size: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename_all = "camelCase")]
    DmcSeries {
        call_to_action: Option<CallToAction>,
        content_id: Uuid,
        current_availability: Value,
        encoded_series_id: Value,
        image: Image,
        media_rights: Option<Value>,
        ratings: Value,
        releases: Value,
        series_id: Uuid,
        tags: Value,
        text: Text,
        text_experience_id: Uuid,
        video_art: Value,
    },
    #[serde(rename_all = "camelCase")]
    DmcVideo(Box<DmcVideo>), // large variant is heap allocated
    #[serde(rename_all = "camelCase")]
    StandardCollection {
        call_to_action: Option<CallToAction>,
        collection_group: Value,
        collection_id: Uuid,
        image: Image,
        text: Text,
        video_art: Value,
    },
}

impl Item {
    pub fn image(&self) -> &Image {
        match &self {
            Self::DmcSeries { image, .. } | Self::StandardCollection { image, .. } => image,
            Self::DmcVideo(dmc) => &dmc.image,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DmcVideo {
    call_to_action: Option<CallToAction>,
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
    media_rights: Option<Value>,
    original_language: Value,
    program_id: Uuid,
    program_type: Value,
    ratings: Value,
    releases: Value,
    season_id: Value,
    season_sequence_number: Value,
    series_id: Value,
    tags: Value,
    target_language: Option<Value>,
    text: Text,
    video_art: Value,
    video_id: Value,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Image {
    pub hero_collection: ImageAspectMap,
    pub tile: ImageAspectMap,
    pub background: Option<ImageAspectMap>,
    pub background_details: Option<ImageAspectMap>,
    pub hero_tile: Option<ImageAspectMap>,
    pub title_treatment: Option<ImageAspectMap>,
    pub title_treatment_layer: Option<ImageAspectMap>,
    pub logo: Option<ImageAspectMap>,
    pub logo_layer: Option<ImageAspectMap>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ImageAspectMap(BTreeMap<String, ImageSized>);

impl ImageAspectMap {
    // find the ImageConcrete with the aspect ratio closests to target_aspect
    pub fn get_closest(&self, target_aspect: f64) -> Option<&ImageConcrete> {
        assert!(target_aspect > 0.0);
        assert!(target_aspect < 1000.0);
        self.all_concrete().min_by_key(|a| {
            let err = (target_aspect - a.aspect_ratio().unwrap_or(0.0)).abs();
            (err * 1000.0) as u64
        })
    }

    pub fn all_concrete(&self) -> impl Iterator<Item = &ImageConcrete> {
        self.0.values().map(ImageSized::concrete)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ImageSized {
    Program { default: ImageConcrete },
    Series { default: ImageConcrete },
    Default { default: ImageConcrete },
}

impl ImageSized {
    pub fn concrete(&self) -> &ImageConcrete {
        match &self {
            ImageSized::Program { default }
            | ImageSized::Series { default }
            | ImageSized::Default { default } => default,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageConcrete {
    pub master_height: u32,
    pub master_id: String,
    pub master_width: u32,
    pub url: String,
}

impl ImageConcrete {
    fn aspect_ratio(&self) -> Option<f64> {
        if self.master_height == 0 {
            return None;
        }
        Some(self.master_width as f64 / self.master_height as f64)
    }
}

/// this was always null in sample data
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CallToAction(Value);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Language(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspect() {
        assert_eq!(
            ImageConcrete {
                master_height: 1,
                master_id: "".into(),
                master_width: 50,
                url: "".into(),
            }
            .aspect_ratio(),
            Some(50.0)
        );
        assert_eq!(
            ImageConcrete {
                master_height: 0,
                master_id: "".into(),
                master_width: 50,
                url: "".into(),
            }
            .aspect_ratio(),
            None
        );
    }

    #[test]
    fn get_closest() {
        let imam = ImageAspectMap(
            [
                ImageConcrete {
                    master_height: 1,
                    master_id: "".into(),
                    master_width: 55,
                    url: "".into(),
                },
                ImageConcrete {
                    master_height: 1,
                    master_id: "".into(),
                    master_width: 50,
                    url: "".into(),
                },
                ImageConcrete {
                    master_height: 1,
                    master_id: "".into(),
                    master_width: 45,
                    url: "".into(),
                },
            ]
            .iter()
            .cloned()
            .map(|default| (format!("{:?}", &default), ImageSized::Default { default }))
            .collect(),
        );
        assert_eq!(
            imam.get_closest(50.0).unwrap(),
            &ImageConcrete {
                master_height: 1,
                master_id: "".into(),
                master_width: 50,
                url: "".into(),
            },
        );
        assert_eq!(
            imam.get_closest(30.0).unwrap(),
            &ImageConcrete {
                master_height: 1,
                master_id: "".into(),
                master_width: 45,
                url: "".into(),
            },
        );
        assert_eq!(
            imam.get_closest(60.0).unwrap(),
            &ImageConcrete {
                master_height: 1,
                master_id: "".into(),
                master_width: 55,
                url: "".into(),
            },
        );
    }
}
