use serde::{Deserialize, Serialize};
use serde_nested_with::serde_nested;
use serde_test::{assert_tokens, Token};
use std::collections::BTreeMap;
use std::option::Option as O_p_t_i_o_n;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foo {
    #[serde(with = "rfc3339")]
    pub bar0: OffsetDateTime,
    #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
    pub bar1: std::option::Option<OffsetDateTime>,
    #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
    pub bar2: O_p_t_i_o_n<Option<OffsetDateTime>>,
    #[serde_nested(sub = "OffsetDateTime", serde(with = "time::serde::rfc3339"))]
    pub bar3: Option<BTreeMap<i32, OffsetDateTime>>,
    #[serde_nested(sub = "time::OffsetDateTime", serde(with = "rfc3339"))]
    pub bar4: BTreeMap<i32, time::OffsetDateTime>,
    #[serde_nested(sub = "time::OffsetDateTime", serde(with = "rfc3339"))]
    pub bar5: Vec<(time::OffsetDateTime, time::OffsetDateTime)>,
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
        bar4: {
            let mut map = BTreeMap::new();
            map.insert(1, OffsetDateTime::from_unix_timestamp(1000000000).unwrap());
            map
        },
        bar5: vec![(
            OffsetDateTime::from_unix_timestamp(1000000000).unwrap(),
            OffsetDateTime::from_unix_timestamp(2000000000).unwrap(),
        )],
    };

    assert_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 6 },
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
            Token::Str("bar4"),
            Token::Map { len: Some(1) },
            Token::I32(1),
            Token::Str("2001-09-09T01:46:40Z"),
            Token::MapEnd,
            Token::Str("bar5"),
            Token::Seq { len: Some(1) },
            Token::Tuple { len: 2 },
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("2033-05-18T03:33:20Z"),
            Token::TupleEnd,
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}
