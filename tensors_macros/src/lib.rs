use proc_macro::TokenStream;
use quote::quote;
use syn::{Generics, Ident, LitInt, Token, TypeArray, token::Type};

#[proc_macro]
pub fn einstein(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let a = syn::parse::<LitInt>(input).unwrap();
    let rank = a.base10_parse::<usize>().unwrap();
    let mut generics = vec![];
    let mut dim_ident = vec![];
    for i in 0..rank {
        let dim: Ident = syn::parse_str::<Ident>(&format!("D{}", i)).unwrap();
        dim_ident.push(dim.clone());
        generics.push(quote! {
            const #dim: usize
        });
    }
    let mut type_str = String::new();
    for _ in 0..rank {
        type_str.push('[');
    }
    type_str.push('T');
    for i in 0..rank {
        type_str.push_str(&format!("; D{}]", rank - 1 - i));
    }
    dbg!(&type_str);
    let content_type = syn::parse_str::<TypeArray>(&type_str).unwrap();
    quote! {
        use std::marker::PhantomData;
        pub struct Tensor<#(#generics),*, T> {
            content: #content_type
        }
    }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
