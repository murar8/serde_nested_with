use serde::{Deserialize, Serialize};
use serde_nested_with::serde_nested;
use serde_test::{assert_tokens, Token};
use std::collections::HashMap;

mod i32_as_string {
    use serde::Deserialize;

    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foo {
    #[serde(rename = "baz")]
    #[serde_nested(
        sub = "i32",
        serde(with = "i32_as_string"),
        derive_trait = "PartialEq",
        derive_trait = "Eq",
        derive_trait = "Hash"
    )]
    pub bar: HashMap<i32, i32>,
}

#[test]
fn test_serialize_deserialize_with() {
    let item = Foo {
        bar: {
            let mut map = HashMap::new();
            map.insert(1, 2);
            map
        },
    };

    assert_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 1 },
            Token::Str("baz"),
            Token::Map { len: Some(1) },
            Token::Str("1"),
            Token::Str("2"),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}
