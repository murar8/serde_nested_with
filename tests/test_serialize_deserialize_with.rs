use serde::{Deserialize, Serialize};
use serde_nested_with::serde_nested;
use serde_test::{assert_tokens, Token};
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foo {
    #[serde_nested(
        sub = "OffsetDateTime",
        serde(serialize_with = "rfc3339::serialize", deserialize_with = "rfc3339::deserialize")
    )]
    pub bar: Option<OffsetDateTime>,
}

#[test]
fn test_serialize_deserialize_with() {
    let item = Foo { bar: OffsetDateTime::from_unix_timestamp(1000000000).ok() };

    assert_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 1 },
            Token::Str("bar"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::StructEnd,
        ],
    );
}
