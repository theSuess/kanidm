#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]

mod entry;

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn qs_test(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::qs_test(&args, item, true)
}

#[proc_macro_attribute]
pub fn qs_test_no_init(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::qs_test(&args, item, false)
}

#[proc_macro_attribute]
pub fn idm_test(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::idm_test(&args, item)
}
