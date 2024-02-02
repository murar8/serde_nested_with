use serde::Deserialize;
use serde_test::{assert_de_tokens, Token};
use serde_with_nested::serde_nested_with;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[serde_nested_with]
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Foo {
    #[serde_nested_with(substitute = "Option<_>", deserialize_with = "rfc3339::deserialize")]
    pub bar1: Option<OffsetDateTime>,
    #[serde_nested_with(substitute = "Vec<Option<_>>", deserialize_with = "rfc3339::deserialize")]
    pub bar2: Vec<Option<OffsetDateTime>>,
}

#[test]
fn test_serialize_with() {
    let item = Foo {
        bar1: OffsetDateTime::from_unix_timestamp(1000000000).ok(),
        bar2: vec![OffsetDateTime::from_unix_timestamp(1000000000).ok()],
    };

    assert_de_tokens(
        &item,
        &[
            Token::Struct { name: "Foo", len: 2 },
            Token::Str("bar1"),
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::Str("bar2"),
            Token::Seq { len: Some(1) },
            Token::Some,
            Token::Str("2001-09-09T01:46:40Z"),
            Token::SeqEnd,
            Token::StructEnd,
        ],
    );
}
