use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use typed_builder::TypedBuilder;

use std::borrow::Cow;
use std::fmt::Debug;

// -------------------------------------------------------------------------------------------------
// # QueryINfo

/// APIのクエリにjsonとして渡す情報(シリアライズ専用)
#[derive(TypedBuilder, Serialize, Clone, PartialEq, Default)]
pub struct QueryInfoRef<'a, T>
where
    T: 'a + ToOwned,
{
    #[builder(default, setter(strip_option))]
    pub reference: Option<Cow<'a, T>>,
}

/// 明示的にデシリアライズするだけの関数．未定義をNoneとすることを防ぐ．
fn deserialize_explicitly<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Deserialize::deserialize(deserializer)
}

/// APIのクエリにjsonとして渡す情報(デシリアライズ専用)
#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
pub struct QueryInfo<T>
where
    T: DeserializeOwned,
{
    #[serde(deserialize_with = "deserialize_explicitly")]
    pub reference: Option<T>,
}

impl<'a, T> From<QueryInfoRef<'a, T>> for QueryInfo<T>
where
    T: ToOwned<Owned = T> + DeserializeOwned,
{
    fn from(value: QueryInfoRef<'a, T>) -> Self {
        let QueryInfoRef { reference } = value;

        QueryInfo {
            reference: reference.map(Cow::into_owned),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{QueryInfo, QueryInfoRef};
    use domain::episode::Episode;
    use std::borrow::Cow;

    use fake::{Fake, Faker};

    #[test]
    fn test_serialize_deserialize_json() {
        let episode = Faker.fake::<Episode>();

        let query_info = QueryInfoRef::builder()
            .reference(Cow::Borrowed(&episode))
            .build();

        let query_info_json = serde_json::to_string(&query_info).unwrap();

        let query_info_de = serde_json::from_str::<QueryInfo<Episode>>(&query_info_json).unwrap();
        assert_eq!(Into::<QueryInfo<Episode>>::into(query_info), query_info_de);
    }

    #[test]
    fn test_deserialize_fail() {
        let episode = Faker.fake::<Episode>();
        let de_res = serde_json::from_str::<QueryInfo<Episode>>(
            serde_json::to_string(&episode).unwrap().as_str(),
        );

        assert!(matches!(de_res, Err(_)))
    }
}
