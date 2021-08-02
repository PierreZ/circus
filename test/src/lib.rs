#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Allow injection of a random seed upon a test. Can be overloaded with environment var `DETERMINISTIC_SEED`.
//!
//! ## Example:
//! ```rust
//! use circus_test::with_random_seed;
//! #[with_random_seed]
//! #[test]
//! fn random_seed(seed: u64) {
//!     println!("{}", seed);
//! }
//! ```

use proc_macro::TokenStream;
use syn::{AttributeArgs, ItemFn};

/// Allow injection of a random seed upon a test. Can be overloaded with environment var `DETERMINISTIC_SEED`.
///
/// ## Example:
/// ```rust
/// use circus_test::with_random_seed;
/// #[with_random_seed]
/// #[test]
/// fn random_seed(seed: u64) {
///     println!("{}", seed);
/// }
/// ```
#[proc_macro_attribute]
pub fn with_random_seed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attributes = syn::parse_macro_input!(attr as AttributeArgs);
    let input = syn::parse_macro_input!(item as ItemFn);

    wrap_test_function(&input)
}

fn wrap_test_function(input: &ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;
    let block = &input.block;
    let attrs = &input.attrs;

    let body = quote::quote! {
        let seed: u64 = match std::env::var("DETERMINISTIC_SEED") {
            Ok(val) => match val.parse::<u64>() {
                Ok(seed) => seed,
                Err(e) => panic!("could not parse '{}' as an u64: {}", val, e),
            },
            Err(_) => rand::random(),
        };
        #block

    };

    quote::quote!(
        #(#attrs)*
        fn #fn_name() {
            #body
        }
    )
    .into()
}
