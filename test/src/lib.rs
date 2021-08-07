#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Allow injection of a random seed upon a test. Can be overloaded with environment var `DETERMINISTIC_SEED`.
//!
//! ## With random seed:
//! ```rust
//! use circus_test::with_random_seed;
//!
//! #[with_random_seed]
//! #[test]
//! fn random_seed(seed: u64) {
//!     println!("{}", seed);
//! }
//! ```
//! ## With fixed seed:
//! ```rust
//! use circus_test::with_seed;
//!
//! #[with_seed(42)]
//! #[test]
//! fn random_seed(seed: u64) {
//!     println!("{}", seed);
//! }
//! ```

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, LitInt};

#[derive(Debug)]
#[doc(hidden)]
struct Seed {
    value: Option<u64>,
}

impl Parse for Seed {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lit: LitInt = input.parse()?;
        let value = lit.base10_parse::<u64>()?;
        Ok(Seed { value: Some(value) })
    }
}

/// Allow injection of a random seed upon a test. Can be overloaded with environment var `DETERMINISTIC_SEED`.
///
/// ## Example:
/// ```rust
/// use circus_test::with_random_seed;
///
/// #[with_random_seed]
/// #[test]
/// fn random_seed(seed: u64) {
///     println!("{}", seed);
/// }
/// ```
#[proc_macro_attribute]
pub fn with_random_seed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    wrap_test_function(&input, None)
}

/// Allow injection of a fixed seed upon a test.
///
/// ## Example:
/// ```rust
/// use circus_test::with_seed;
///
/// #[with_seed(42)]
/// #[test]
/// fn random_seed(seed: u64) {
///     assert_eq!(42, seed);
/// }
/// ```
#[proc_macro_attribute]
pub fn with_seed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = syn::parse_macro_input!(attr as Seed);
    let input = syn::parse_macro_input!(item as ItemFn);

    wrap_test_function(&input, attributes.value)
}

fn wrap_test_function(input: &ItemFn, seed: Option<u64>) -> TokenStream {
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let attrs = &input.attrs;

    let body = match seed {
        None => {
            quote::quote! {
                let seed: u64 = match std::env::var("DETERMINISTIC_SEED") {
                    Ok(val) => match val.parse::<u64>() {
                        Ok(seed) => seed,
                        Err(e) => panic!("could not parse '{}' as an u64: {}", val, e),
                    },
                    Err(_) => rand::random(),
                };
                #block

            }
        }
        Some(seed) => {
            quote::quote! {
                let seed: u64 = #seed;
                #block

            }
        }
    };

    quote::quote!(
        #(#attrs)*
        fn #fn_name() {
            #body
        }
    )
    .into()
}

#[cfg(test)]
mod tests {
    use crate::Seed;

    #[test]
    fn test_seed() {
        let seed = Seed { value: None };
        dbg!(seed);
    }
}
