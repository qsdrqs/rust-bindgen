use proc_macro::{TokenStream, Ident, Span};
use quote::{quote, ToTokens, __private::Span as QSpan};
use syn::{self, parse::Parser, DeriveInput, token::Token, spanned};

#[proc_macro_derive(Shadow)]
pub fn shadow(item: TokenStream) -> TokenStream {
    let mut output: TokenStream = item.clone();
    let ast: DeriveInput = syn::parse(item).unwrap();
    let mut shadowStruct = ast.clone();
    let shadowStructName = format!("Shadow{}", ast.ident);
    shadowStruct.ident = syn::Ident::new(&shadowStructName, QSpan::call_site());
    let mut data = &mut shadowStruct.data;
    let mut fields = match &mut data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("#[foo] can only be applied to structs"),
    };
    let fields_vec = &mut fields.named;
    for field in fields_vec {
        let field_type = &mut field.ty;
        println!();
        println!("{:?}", field_type);
        match field_type {
            syn::Type::Path(path) => {
                let ident = &path.path.segments.to_token_stream().to_string();
                if ident.to_string().contains("long") {
                    field.ty = syn::parse2(quote!{ i32 }).unwrap();
                }
            },
            syn::Type::Ptr(ptr) => {
                field.ty = syn::parse2(quote!{ i32 }).unwrap();
            },
            others => panic!("field of #[foo] has type of {:?} that can't be handled yet", others),
        };
    }

    let res = quote!(#shadowStruct);
    println!();
    println!("{:?}", res.to_string());
    return res.into();
}