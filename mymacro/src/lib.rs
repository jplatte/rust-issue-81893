use proc_macro::TokenStream;

#[proc_macro]
pub fn do_stuff(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
