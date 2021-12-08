use std::str::FromStr;

use proc_macro::TokenStream;
use twofloat::TwoFloat;

#[proc_macro]
pub fn twofloat(input: TokenStream) -> TokenStream {
    let literal = syn::parse_macro_input!(input as syn::LitStr);
    match TwoFloat::from_str(&literal.value()) {
        Ok(value) =>
            format!(
                concat!(
                    "<::twofloat::TwoFloat as ::core::convert::TryFrom<(f64, f64)>>::try_from(({0:?}f64, {1:?}f64))",
                    ".unwrap_or_else(|_| ::twofloat::TwoFloat::new_add({0:?}f64, {1:?}f64))"),
                value.hi(),
                value.lo())
            .parse()
            .expect("Could not create token stream for TwoFloat"),
        Err(e) => syn::Error::new(literal.span(), format!("twofloat! failed: {}", e))
            .to_compile_error()
            .into(),
    }
}
