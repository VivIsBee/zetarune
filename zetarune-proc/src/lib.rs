use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
};

extern crate proc_macro;

struct ImageItem {
    file: LitStr,
    #[allow(unused)]
    fat_arrow: Token![=>],
    id: LitStr,
}

impl Parse for ImageItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file = input.parse()?;
        let fat_arrow = input.parse()?;
        let id = input.parse()?;

        Ok(Self {
            file,
            fat_arrow,
            id,
        })
    }
}

/// Load images from a provided path (relative to the crate root), compress them, and wrap a `CompressedStaticAssetProvider` around them. Example:
/// ```ignore
/// compressed_sprites! {
///     "sprites/player.png" => "player:sheet",
/// }
/// ```
#[proc_macro]
pub fn compressed_sprites(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input with syn::punctuated::Punctuated::<ImageItem, Token![,]>::parse_separated_nonempty);

    let base = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let mut out_s = vec![];
    let mut out_b = vec![];

    for item in input {
        let contents = match std::fs::read(base.clone().join(item.file.value())).map_err(|v| {
            let v = v.to_string();
            quote! { compile_error!(#v) }
        }) {
            Ok(v) => v,
            Err(err) => return err.into(),
        };

        let image = match image::load_from_memory(&contents).map_err(|v| {
            let v = v.to_string();
            quote! { compile_error!(#v) }
        }) {
            Ok(v) => v,
            Err(err) => return err.into(),
        };

        let mut out_bytes = Vec::new();

        match image
            .write_with_encoder(image::codecs::qoi::QoiEncoder::new(&mut out_bytes))
            .map_err(|v| {
                let v = v.to_string();
                quote! { compile_error!(#v) }
            }) {
            Ok(_) => {}
            Err(err) => return err.into(),
        };

        out_s.push(item.id);
        out_b.push(out_bytes);
    }

    quote_spanned! {proc_macro2::Span::call_site()=>
        const {
            const __COMPRESSED_SPRITES: &[(&str, &[u8])] = &[#((#out_s, &[#(#out_b),*])),*];
            ::zetarune::resources::CompressedStaticAssetProvider::new(__COMPRESSED_SPRITES)
        }
    }
    .into()
}
