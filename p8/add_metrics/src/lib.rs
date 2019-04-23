#![recursion_limit="256"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn::{
    Ident, ItemFn, 
};
 
#[proc_macro_attribute]
pub fn flush_metrics(args: TokenStream, function: TokenStream) -> TokenStream {
    assert_eq!(args.is_empty(), false);

    let mut function = parse_macro_input!(function as ItemFn);

    let body = function.block;
    let fn_name = format!("{}", args);
    let fn_name = Ident::new(&fn_name, proc_macro2::Span::call_site());
    function.block = Box::new(parse_quote!({
        let __wrap = (move || {
            #fn_name();
            #body
        })();
        __wrap
    }));

    TokenStream::from(quote!(#function))
}