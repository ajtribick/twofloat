pub mod common;

#[cfg(feature = "serde_support")]
pub mod tests {
    use super::common::*;
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};
    use twofloat::{no_overlap, TwoFloat};

    #[test]
    fn serialize_test() {
        repeated_test(|| {
            let source = get_twofloat();
            assert_tokens(
                &source,
                &[
                    Token::Tuple { len: 2 },
                    Token::F64(source.hi()),
                    Token::F64(source.lo()),
                    Token::TupleEnd,
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
                    Token::Tuple { len: 2 },
                    Token::F64(hi),
                    Token::F64(lo),
                    Token::TupleEnd,
                ],
                "invalid TwoFloat conversion",
            );
        });
    }
}
