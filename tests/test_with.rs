use serde::{Deserialize, Serialize};
use serde_test::{assert_tokens, Token};
use serde_with_nested::serde_nested_with;
use std::collections::BTreeMap;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested_with]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foo {
    #[serde(with = "rfc3339")]
    pub bar0: OffsetDateTime,
    #[serde_nested_with(substitute = "Option<_>", with = "rfc3339")]
    pub bar1: Option<OffsetDateTime>,
    #[serde_nested_with(substitute = "Option<Option<_>>", with = "rfc3339")]
    pub bar2: Option<Option<OffsetDateTime>>,
    #[serde_nested_with(substitute = "Option<BTreeMap<i32, _>>", with = "rfc3339")]
    pub bar3: Option<BTreeMap<i32, OffsetDateTime>>,
}

#[test]
fn test_with() {
    let item = Foo {
        bar0: OffsetDateTime::from_unix_timestamp(1000000000).unwrap(),
        bar1: OffsetDateTime::from_unix_timestamp(1000000000).ok(),
        bar2: OffsetDateTime::from_unix_timestamp(1000000000).ok().into(),
        bar3: {
            let mut map = BTreeMap::new();
            map.insert(1, OffsetDateTime::from_unix_timestamp(1000000000).unwrap());
            map.insert(2, OffsetDateTime::from_unix_timestamp(2000000000).unwrap());
            map.insert(3, OffsetDateTime::from_unix_timestamp(3000000000).unwrap());
            Some(map)
        },
    };

    assert_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 4 }, //:w
            Token::Str("bar0"),
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("bar1"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("bar2"),
            Token::Some,
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("bar3"),
            Token::Some,
            Token::Map { len: Some(3) },
            Token::I32(1),
            Token::Str("2001-09-09T01:46:40Z"),
            Token::I32(2),
            Token::Str("2033-05-18T03:33:20Z"),
            Token::I32(3),
            Token::Str("2065-01-24T05:20:00Z"),
            Token::MapEnd,
            Token::StructEnd,
        ],
    );
}
