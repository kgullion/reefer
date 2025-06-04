use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, punctuated::Punctuated};

mod build;
mod cas;
mod geometry;
mod sort;
mod traits;

/// helper for building syn::Error errors
macro_rules! err {
    ($expr:expr, $msg:expr) => {{
        #[allow(unused_imports)]
        use syn::spanned::Spanned;
        syn::Error::new(($expr).span(), $msg)
    }};
    ($msg:expr) => {
        syn::Error::new(proc_macro2::Span::call_site(), $msg)
    };
}
pub(crate) use err;

/// macro for naming shape types
///     Mv!(e0, e1, e2) -> Mv_e0_e1_e2
#[proc_macro]
#[allow(non_snake_case)]
pub fn Mv(input: TokenStream) -> TokenStream {
    match build::mv_ty_path(parse_macro_input!(input)) {
        Ok(path) => path.to_token_stream().into(),
        Err(e) => e.to_compile_error().to_token_stream().into(),
    }
}

/// builds an instance of a specific multivector. Note that because the struct type name is dependant on
/// the order and orientation of the blades, they must match a predefined shape.
///     mv![e0: 3, e1: 1, e2: 2] -> Mv_e0_e1_e2{e0: 3, e1: 1, e2: 2}
#[proc_macro]
pub fn mv(input: TokenStream) -> TokenStream {
    match build::mv(parse_macro_input!(input with Punctuated::parse_separated_nonempty)) {
        Ok(struct_) => struct_.to_token_stream().into(),
        Err(e) => e.to_compile_error().to_token_stream().into(),
    }
}

/// specifies module as a geometric algebra
#[proc_macro_attribute]
pub fn algebraic(attrs: TokenStream, input: TokenStream) -> TokenStream {
    match build::algebraic(parse_macro_input!(attrs), parse_macro_input!(input)) {
        Ok(mod_) => {
            // println!(
            //     "{}",
            //     prettyplease::unparse(&syn::File {
            //         shebang: None,
            //         attrs: vec![],
            //         items: vec![syn::Item::Mod(mod_.clone())],
            //     })
            // );
            mod_.to_token_stream().into()
        }
        Err(e) => e.to_compile_error().into(),
    }
}

/// declare the value for the square of an axis
#[proc_macro]
pub fn square(_: TokenStream) -> TokenStream {
    err!("square! used outside of reefer module")
        .into_compile_error()
        .into()
}
/// declare a shape family
#[proc_macro]
pub fn shape(_: TokenStream) -> TokenStream {
    err!("shape! used outside of reefer module")
        .into_compile_error()
        .into()
}
