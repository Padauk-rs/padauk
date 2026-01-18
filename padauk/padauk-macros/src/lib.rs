extern crate proc_macro;

use quote::quote;

#[proc_macro_attribute]
pub fn main(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // 1. Parse the input into a syn Syntax Tree
    // parse_macro_input! handles the conversion from proc_macro::TokenStream internally
    // let input = parse_macro_input!(item as ItemFn);
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let name = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;

    // 2. Build the output using quote!
    // This generates a proc_macro2::TokenStream
    let expanded = quote! {
        #(#attrs)*
        #[allow(dead_code)]
        #vis fn #name() {
            let app = (|| #block)();
            padauk::start_app(app);
        }

        #[uniffi::export]
        pub fn padauk_init() {
            #name();
        }
    };

    // 3. Convert proc_macro2::TokenStream back to proc_macro::TokenStream
    proc_macro::TokenStream::from(expanded)
}
