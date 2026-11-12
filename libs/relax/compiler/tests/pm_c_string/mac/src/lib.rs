use proc_macro::{Literal, TokenStream};

/// Expands to a C string literal: `Literal::c_string` through the JIT proc-macro host.
#[proc_macro]
pub fn hello(_input: TokenStream) -> TokenStream {
    Literal::c_string(c"hello").to_string().parse().unwrap()
}
