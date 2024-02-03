use serde::Serialize;
use serde_nested_with::serde_nested;
use serde_test::{assert_ser_tokens, Token};
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Foo {
    #[serde_nested(sub = "OffsetDateTime", serde(serialize_with = "rfc3339::serialize"))]
    pub bar1: Option<OffsetDateTime>,
    #[serde_nested(sub = "OffsetDateTime", serde(serialize_with = "rfc3339::serialize"))]
    pub bar2: Option<Option<OffsetDateTime>>,
}

#[test]
fn test_serialize_with() {
    let item = Foo {
        bar1: OffsetDateTime::from_unix_timestamp(1000000000).ok(),
        bar2: OffsetDateTime::from_unix_timestamp(1000000000).ok().into(),
    };

    assert_ser_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 2 },
            Token::Str("bar1"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("bar2"),
            Token::Some,
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::StructEnd,
        ],
    );
}
