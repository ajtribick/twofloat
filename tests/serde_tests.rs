pub mod common;

#[cfg(feature = "serde_support")]
pub mod serde_tests {
    use super::common::*;
    use twofloat::{TwoFloat, no_overlap};
    use serde_test::{Token, assert_tokens, assert_de_tokens_error};

    randomized_test!(serialize_test, |rng: F64Rand| {
        let source = get_twofloat(rng);
        assert_tokens(&source, &[
            Token::Tuple { len: 2 },
            Token::F64(source.hi()),
            Token::F64(source.lo()),
            Token::TupleEnd,
        ]);
    });

    randomized_test!(deserialize_invalid_test, |rng: F64Rand| {
        let (hi, lo) = get_valid_pair(rng, |x, y| !no_overlap(x, y));
        assert_de_tokens_error::<TwoFloat>(
            &[
                Token::Tuple { len: 2 },
                Token::F64(hi),
                Token::F64(lo),
                Token::TupleEnd,
            ],
            "invalid TwoFloat conversion"
        );
    });
}
