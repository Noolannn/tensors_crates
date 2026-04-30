use std::marker::PhantomData;

use proc_macro::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{AngleBracketedGenericArguments, ConstParam, Expr, ExprConst, GenericArgument, GenericParam, Generics, Ident, LitInt, Token, Type, TypeArray, TypeParam, bracketed, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Bracket, Comma, Gt, Lt, Paren}};

// Support for A[a, b] = B[a, c] * C[c, b]
// for a in 0..DIM0 {
//     for b in 0..DIM1 {
//         let mut sum = 0;
//         for c in 0..DIM2 {
//             sum += B[a][c] * C[c][b];
//         }
//         A[a][b] = sum;
//     }
// }

// F represent the number of free indices
struct Einstein<const F: usize, T> {
    lhs: TensorComp<F, T>,
    rhs: Vec<TensorProd<F, T>>
}

struct TensorProd<const F: usize, T> {
    coef: Expr,
    tensors: Vec<TensorComp<F, T>>
}

#[derive(Debug)]
struct TensorComp<const F: usize, T> {
    _phantom: PhantomData<T>,
    tensor_ident: Ident,
    indices: Punctuated<TensorIndex, Comma>
}

impl<const F: usize, T> ToTokens for TensorComp<F, T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.tensor_ident.clone());
        for index in self.indices.iter().cloned() {
            quote! { [#index] }.to_tokens(tokens);
        }
    }
}

impl<const F: usize, T> Parse for TensorComp<F, T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tensor_ident = input.parse::<Ident>()?;
        let content;
        bracketed!(content in input);
        // let mut indices = vec![];
        // while let Ok(index) = content.parse::<TensorIndex>() {
        //     indices.push(index);
        //     if let Err(_) = content.parse::<Token![,]>() {
        //         break;
        //     }
        // }
        let indices = content.parse_terminated(TensorIndex::parse, Token![,])?;

        if indices.is_empty() {
            panic!()
        }

        if indices.iter().all(|e| !e.is_var()) { // If all elements are constants, we don't actually care of the tensor, because it won't be sumed over
            return Err(input.error("All indices are constants"));
        }
        Ok(Self {
            _phantom: PhantomData,
            tensor_ident: tensor_ident,
            indices: indices,
        })
    }
}

#[derive(Debug, Clone)]
enum TensorIndex {
    Cst(LitInt),
    Var(Ident)
}

impl ToTokens for TensorIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            TensorIndex::Cst(lit_int) => lit_int.to_tokens(tokens),
            TensorIndex::Var(ident) => ident.to_tokens(tokens),
        }
    }
}

impl TensorIndex {
    fn is_var(&self) -> bool {
        if let TensorIndex::Var(_) = self {
            return true;
        } else {
            return false;
        }
    }
}

impl Parse for TensorIndex {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(litint) = input.parse::<LitInt>() {
            return Ok(TensorIndex::Cst(litint));
        } else {
            let ident = input.parse::<Ident>()?;
            return Ok(TensorIndex::Var(ident));
        }
    }
}

#[proc_macro]
pub fn einstein(input: TokenStream) -> TokenStream {
    let ident = syn::parse::<Ident>(input).unwrap();
    dbg!(ident);
    quote! {}.into()
}

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let a = syn::parse::<LitInt>(input).unwrap();
    let rank = a.base10_parse::<usize>().unwrap();

    let dim_idents = (0..rank).map(|i| {
        Ident::new(&format!("D{}", i), proc_macro2::Span::call_site())
    }).collect::<Vec<Ident>>();

    // NEW CODE (can be removed if it doesn't work)
    // Creation of the generic signature of the type
    let mut generics = Generics::default();
    for i in 0..rank {
        let const_name = dim_idents[i].clone();
        // Create the const arg
        let generic_param = syn::parse::<GenericParam>(quote! {const #const_name: usize}.into()).unwrap();
        generics.params.push(generic_param);
    }
    // Create the type arg
    generics.params.push(syn::parse::<GenericParam>(quote! {T}.into()).unwrap());
    // END OF NEW CODE

    let last_dim = dim_idents.last().unwrap().clone();
    let mut content = syn::parse::<TypeArray>(quote! {[T; #last_dim]}.into()).unwrap();
    for i in 1..rank {
        let current_dim = dim_idents[rank - 1 - i].clone();
        content = syn::parse::<TypeArray>(quote! {[#content; #current_dim]}.into()).unwrap();
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
    // TOKEN GEN FOR THE NEW CODE
    quote! {
        pub struct Tensor #generics {
            content: #content
        }
    }.into()
}

#[proc_macro]
pub fn test_macro(input: TokenStream) -> TokenStream {
    let a: TokenStream = quote! {
        tensor[a, 1]
    }.into();
    let parse_res = syn::parse::<Generics>(input).unwrap();
    // let parse_res = syn::parse::<TensorComp<2, i32>>(input).unwrap();
    let dbg_str = format!("{:?}", parse_res);
    quote! {
        println!("{}", #dbg_str);
    }.into()
}

#[cfg(test)]
mod tests {
    use syn::parse;

    use super::*;

    #[test]
    fn it_works() {
    }
}
