#![cfg(feature = "serde")]

#[macro_use]
pub mod common;

use common::*;
use serde_test::{assert_de_tokens_error, assert_tokens, Token};
use twofloat::{no_overlap, TwoFloat};

#[test]
fn serialize_test() {
    repeated_test(|| {
        let source = get_twofloat();
        assert_tokens(
            &source,
            &[
                Token::Struct {
                    name: "TwoFloat",
                    len: 2,
                },
                Token::Str("hi"),
                Token::F64(source.hi()),
                Token::Str("lo"),
                Token::F64(source.lo()),
                Token::StructEnd,
            ],
        );
    });
}

#[test]
fn deserialize_invalid_test() {
    repeated_test(|| {
        let (hi, lo) = get_valid_pair(|x, y| !no_overlap(x, y));
        assert_de_tokens_error::<TwoFloat>(
            &[
                Token::Struct {
                    name: "TwoFloat",
                    len: 2,
                },
                Token::Str("hi"),
                Token::F64(hi),
                Token::Str("lo"),
                Token::F64(lo),
                Token::StructEnd,
            ],
            "invalid TwoFloat conversion",
        );
    });
}
