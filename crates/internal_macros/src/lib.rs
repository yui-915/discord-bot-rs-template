use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

#[proc_macro]
pub fn commands(_input: TokenStream) -> TokenStream {
    let base_path = Path::new("src").join("commands");

    let names = base_path.read_dir().expect("read_dir").map(|path| {
        path.expect("path")
            .file_name()
            .to_str()
            .expect("file_name .. to_str")
            .split('.')
            .next()
            .expect("split .. next")
            .to_string()
    });

    let parts = names.map(|name| {
        let path = format!("../commands/{}.rs", name);
        let name = format_ident!("{}", name);
        quote! {{
            #[path = #path]
            mod #name;
            #name::#name()
        },}
    });

    quote! {vec![#(#parts)*]}.into()
}
