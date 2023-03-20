use domain::{
    episode::Episode,
    movie_clip::MovieClip,
    video::{Kirinuki, Original, Video},
};

use serde::{Deserialize, Deserializer, Serialize};
use typed_builder::TypedBuilder;

// -------------------------------------------------------------------------------------------------
// # QueryINfo

/// APIのクエリにjsonとして渡す情報
#[derive(TypedBuilder, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct QueryInfo {
    #[serde(deserialize_with = "deserialize_explicitly")]
    #[builder(default, setter(strip_option))]
    pub reference_episode: Option<Episode>,

    #[serde(deserialize_with = "deserialize_explicitly")]
    #[builder(default, setter(strip_option))]
    pub reference_movie_clip: Option<MovieClip>,

    #[serde(deserialize_with = "deserialize_explicitly")]
    #[builder(default, setter(strip_option))]
    pub reference_original: Option<Video<Original>>,

    #[serde(deserialize_with = "deserialize_explicitly")]
    #[builder(default, setter(strip_option))]
    pub reference_kirinuki: Option<Video<Kirinuki>>,
}

/// 明示的にデシリアライズするだけの関数．未定義をNoneとすることを防ぐ．
fn deserialize_explicitly<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Deserialize::deserialize(deserializer)
}

#[cfg(test)]
mod test {
    use super::QueryInfo;
    use domain::episode::Episode;

    use fake::{Fake, Faker};

    #[test]
    fn test_serialize_deserialize_json() {
        let episode = Faker.fake::<Episode>();

        let query_info = QueryInfo::builder().reference_episode(episode).build();

        let query_info_json = serde_json::to_string(&query_info).unwrap();

        let query_info_de = serde_json::from_str(&query_info_json).unwrap();
        assert_eq!(query_info, query_info_de);
    }

    #[test]
    fn test_deserialize_fail() {
        let episode = Faker.fake::<Episode>();
        let de_res =
            serde_json::from_str::<QueryInfo>(serde_json::to_string(&episode).unwrap().as_str());

        assert!(matches!(de_res, Err(_)))
    }
}
