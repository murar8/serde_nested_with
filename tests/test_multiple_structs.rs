use serde::{Deserialize, Serialize};
use serde_nested_with::serde_nested;
use serde_test::{assert_tokens, Token};
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Foo {
    #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
    pub baz: Option<OffsetDateTime>,
}

#[serde_nested]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bar {
    #[serde_nested(sub = "OffsetDateTime", serde(with = "rfc3339"))]
    pub baz: Option<OffsetDateTime>,
}

#[test]
fn test_multiple_structs() {
    let item1 = Foo { baz: OffsetDateTime::from_unix_timestamp(1000000000).ok() };
    let item2 = Bar { baz: OffsetDateTime::from_unix_timestamp(1000000000).ok() };

    assert_tokens(
        &item1,
        &[
            Token::Struct { name: "Foo", len: 1 },
            Token::Str("baz"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::StructEnd,
        ],
    );
    assert_tokens(
        &item2,
        &[
            Token::Struct { name: "Bar", len: 1 },
            Token::Str("baz"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::StructEnd,
        ],
    );
}
