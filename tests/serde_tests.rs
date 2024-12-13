#![cfg(feature = "serde")]

#[macro_use]
pub mod common;

use common::*;
use serde::de::Unexpected;
use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};
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
fn deserialize_map() {
    repeated_test(|| {
        let value = get_twofloat();
        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TwoFloat",
                    len: 2,
                },
                Token::Str("hi"),
                Token::F64(value.hi()),
                Token::Str("lo"),
                Token::F64(value.lo()),
                Token::StructEnd,
            ],
        );
    });
}

#[test]
fn deserialize_seq() {
    repeated_test(|| {
        let value = get_twofloat();
        assert_de_tokens(
            &value,
            &[
                Token::Seq { len: Some(2) },
                Token::F64(value.hi()),
                Token::F64(value.lo()),
                Token::SeqEnd,
            ],
        );
    });
}

#[test]
fn deserialize_too_short_seq_test() {
    let hi = get_valid_f64(f64::is_finite);
    assert_de_tokens_error::<TwoFloat>(
        &[Token::Seq { len: Some(1) }, Token::F64(hi), Token::SeqEnd],
        "invalid length 1, expected struct TwoFloat",
    );
}

#[test]
fn deserialize_overlapping_seq_test() {
    repeated_test(|| {
        let (hi, lo) = get_valid_pair(|x, y| !no_overlap(x, y));
        let expected_error = format!(
            "invalid value: {}, expected non-overlapping low word",
            Unexpected::Float(lo)
        );
        assert_de_tokens_error::<TwoFloat>(
            &[
                Token::Seq { len: Some(2) },
                Token::F64(hi),
                Token::F64(lo),
                Token::SeqEnd,
            ],
            &expected_error,
        );
    });
}

#[test]
fn deserialize_missing_lo_test() {
    let hi = get_valid_f64(f64::is_finite);
    assert_de_tokens_error::<TwoFloat>(
        &[
            Token::Struct {
                name: "TwoFloat",
                len: 1,
            },
            Token::Str("hi"),
            Token::F64(hi),
            Token::StructEnd,
        ],
        "missing field `lo`",
    );
}

#[test]
fn deserialize_missing_hi_test() {
    let lo = get_valid_f64(f64::is_finite);
    assert_de_tokens_error::<TwoFloat>(
        &[
            Token::Struct {
                name: "TwoFloat",
                len: 1,
            },
            Token::Str("lo"),
            Token::F64(lo),
            Token::StructEnd,
        ],
        "missing field `hi`",
    );
}

#[test]
fn deserialize_overlapping_map_test() {
    repeated_test(|| {
        let (hi, lo) = get_valid_pair(|x, y| !no_overlap(x, y));
        let expected_error = format!(
            "invalid value: {}, expected non-overlapping low word",
            Unexpected::Float(lo)
        );
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
            &expected_error,
        );
    });
}
