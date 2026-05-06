use std::marker::PhantomData;

use proc_macro::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{AngleBracketedGenericArguments, BinOp, Block, ConstParam, Expr, ExprArray, ExprAssign, ExprBinary, ExprBlock, ExprConst, ExprForLoop, ExprIndex, ExprRepeat, GenericArgument, GenericParam, Generics, Ident, LitInt, Stmt, Token, Type, TypeArray, TypeParam, bracketed, parenthesized, parse::{Parse, ParseStream}, punctuated::Punctuated, token::{Bracket, Comma, Gt, Lt, Paren, Plus}};

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

#[derive(Debug, Clone)]
struct TensorComp2 {
    tensor_ident: Ident,
    indices: Punctuated<TensorIndex, Comma>
}

impl Parse for TensorComp2 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tensor_ident = input.parse::<Ident>()?;
        let content;
        bracketed!(content in input);
        let inner;
        bracketed!(inner in content);
        let indices = inner.parse_terminated(TensorIndex::parse, Token![,])?;
        Ok(
            Self {
                tensor_ident,
                indices
            }
        )
    }
}

impl ToTokens for TensorComp2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tensor_ident = self.tensor_ident.clone();
        let indices = self.indices.clone();
        quote! {#tensor_ident[[#indices]]}.to_tokens(tokens);
    }
}

struct Einstein2 {
    lhs: TensorComp2,
    rhs: Expr,
    eq_indices: Vec<(Ident, Ident, usize)>, // (index name, tensor ident associated (for sum range), index of the index in the tensor)
    sum_indices: Vec<(Ident, Ident, usize)>
}

impl ToTokens for Einstein2 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {

        let rhs = self.rhs.clone();
        let lhs = self.lhs.clone();
        let mut sum_for_loop = syn::parse::<ExprForLoop>(quote! {
            for _ in 0..1 {
                sum += #rhs;
            }
        }.into()).unwrap();
        for (index_ident, tensor_ident, n) in self.sum_indices.iter() {
            let index_ident = index_ident.clone();
            let tensor_ident = tensor_ident.clone();
            let n = *n;
            let mut inner_for_loop = syn::parse::<ExprForLoop>(quote! {for #index_ident in 0..(#tensor_ident.dims()[#n]) {}}.into()).unwrap();
            inner_for_loop.body.stmts.push(Stmt::Expr(Expr::ForLoop(sum_for_loop), None));
            sum_for_loop = inner_for_loop;
        }

        let mut ext_for_loop = syn::parse::<ExprForLoop>(quote! {
            for _ in 0..1 {
                let mut sum = 0;
                #sum_for_loop
                #lhs = sum;
            }
        }.into()).unwrap();
        let free_indices: usize = self.eq_indices.iter().len();
        for (i, (index_ident, tensor_ident, n)) in self.eq_indices.iter().enumerate() {
            let index_ident = index_ident.clone();
            let tensor_ident = tensor_ident.clone();
            let n = *n;
            let mut inner_for_loop = syn::parse::<ExprForLoop>(quote! {for #index_ident in 0..(#tensor_ident.dims()[#n]) {}}.into()).unwrap();
            inner_for_loop.body.stmts.push(Stmt::Expr(Expr::ForLoop(ext_for_loop), None));
            ext_for_loop = inner_for_loop;
        }
        ext_for_loop.to_tokens(tokens);
    }
}

/// Parse einstein!(res[[a, b]] = gamma0[[a, c]] * gamma1[[c, b]]) by expanding the sum into :
/// for a in 0..4 {
///     for b in 0..4 {
///         let mut rhs = 0;
///         for c in 0..4 {
///             rhs += gamma0[[a, c]] * gamma1[[c, b]];
///         }
///         res[[a, b]] = rhs;
///     }
/// }
/// 
/// Should carefully handle case of various sumed indices in an explicit sum, like for example
/// res[[a, b]] = ta[[a, c]] * tb[[c, b]] + tc[[a, c, d]] * td[[c, d, b]]
#[proc_macro]
pub fn einstein(input: TokenStream) -> TokenStream {
    // let ident = syn::parse::<Ident>(input).unwrap();
    // dbg!(&ident);
    let expr_assign = syn::parse::<ExprAssign>(input).unwrap();
    let left = expr_assign.left;
    let left_tensor = syn::parse::<TensorComp2>(left.to_token_stream().into()).unwrap();
    let right = expr_assign.right;
    let sum_indices = cleaned_indices(*right.clone());
    let mut eq_indices = vec![];
    for (n, index) in left_tensor.indices.iter().enumerate() {
        if let TensorIndex::Var(index_ident) = index {
            eq_indices.push((index_ident.clone(), left_tensor.tensor_ident.clone(), n));
        }
    }
    let einstein = Einstein2 {
        lhs: left_tensor,
        rhs: *right.clone(),
        eq_indices: eq_indices,
        sum_indices: sum_indices
    };
    einstein.to_token_stream().into()
}

/// Gets the list of all indices which should be sumed over
/// Careful, here the usize is the index of the index in the list of tensor indices, it is then used to retrieve the range of summation
fn sum_indices_list(expr: Expr) -> Vec<(Ident, Ident, usize, bool)> {
    let mut res = vec![];
    // Unwrap the parenthesized expression if it is (we use a while loop if there is arbitrarily long imbricated paren)
    let mut expr = expr;
    while let Expr::Paren(paren) = expr {
        expr = *paren.expr;
    }
    // Parse a binary operation or a tensor
    if let Ok(bin_expr) = syn::parse::<ExprBinary>(expr.to_token_stream().into()) {
        match bin_expr.op {
            BinOp::Add(_) => {
                res.append(&mut sum_indices_list(*bin_expr.left));
                res.append(&mut sum_indices_list(*bin_expr.right));
            },
            BinOp::Mul(_) => {
                let mut left_indices = sum_indices_list(*bin_expr.left);
                let mut right_indices = sum_indices_list(*bin_expr.right);
                let mut common_indices = vec![];
                // Checks the common indices in the two lists, which forms the indices which must be summed over (and are labeled "true")
                for (l_ident, l_tensor_ident, l_n, _) in left_indices.iter() {
                    for (r_ident, r_tensor_ident, r_n, _) in right_indices.iter() {
                        if l_ident.to_string() == r_ident.to_string() {
                            common_indices.push((l_ident.clone(), l_tensor_ident.clone(), l_n.clone(), true));
                        }
                    }
                }
                let mut total_indices = vec![];
                total_indices.append(&mut left_indices);
                total_indices.append(&mut right_indices);
                total_indices.append(&mut common_indices);
                res.append(&mut total_indices);
            },
            _ => {
                panic!("Unsupported tensor operation, current expr : {:?}", expr)
            }
        }
    } else { // Parse a tensor or a scalar
        if let Ok(tensor_comp) = syn::parse::<TensorComp2>(expr.to_token_stream().into()) {
            for (n, index) in tensor_comp.indices.iter().enumerate() {
                if let TensorIndex::Var(ident) = index {
                    res.push((ident.clone(), tensor_comp.tensor_ident.clone(), n, false));
                }
            }
        } 
        // else {
        //     panic!("Failed to parse a tensor or a binary operation {:?}", expr)
        // }
    }
    return res;
}

fn cleaned_indices(expr: Expr) -> Vec<(Ident, Ident, usize)> {
    let res = sum_indices_list(expr);
    let mut cleaned = vec![];
    for i in 0..res.len() {
        if res[i].3 {
            cleaned.push((res[i].0.clone(), res[i].1.clone(), res[i].2));
        }
    }
    cleaned.dedup(); // deduplicate the indices
    return cleaned;
}

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    let rank_litint = syn::parse::<LitInt>(input).unwrap();
    let rank = rank_litint.base10_parse::<usize>().unwrap();

    let dim_idents = (0..rank).map(|i| {
        Ident::new(&format!("D{}", i), proc_macro2::Span::call_site())
    }).collect::<Vec<Ident>>();

    // Creation of the generic signature of the type
    let mut generics = Generics::default();
    // Create the generic bind used for impl blocks
    let mut generic_bind = syn::parse::<AngleBracketedGenericArguments>(quote! {<>}.into()).unwrap();
    for i in 0..rank {
        let const_name = dim_idents[i].clone();
        // Create the const arg
        let generic_param = syn::parse::<GenericParam>(quote! {const #const_name: usize}.into()).unwrap();
        generics.params.push(generic_param);
        generic_bind.args.push(syn::parse::<GenericArgument>(quote! {#const_name}.into()).unwrap());
    }
    // Adds the type arg
    generics.params.push(syn::parse::<GenericParam>(quote! {T: Default + Clone + Copy + std::ops::Add + std::ops::Mul}.into()).unwrap());
    generic_bind.args.push(syn::parse::<GenericArgument>(quote! {T}.into()).unwrap());

    let last_dim = dim_idents.last().unwrap().clone();
    // Create content_type, the type of the large array containing all the tensor components
    let mut content_type = syn::parse::<TypeArray>(quote! {[T; #last_dim]}.into()).unwrap();
    // Create the default value of this array
    let mut content = syn::parse::<ExprRepeat>(quote! {[T::default(); #last_dim]}.into()).unwrap();
    for i in 1..rank {
        let current_dim = dim_idents[rank - 1 - i].clone();
        content_type = syn::parse::<TypeArray>(quote! {[#content_type; #current_dim]}.into()).unwrap();
        content = syn::parse::<ExprRepeat>(quote! {[#content; #current_dim]}.into()).unwrap();
    }

    // Create the list type used for indexing the tensor as tensor[[0, 1]] for example
    let list_index_type = syn::parse::<TypeArray>(quote! {[usize; #rank_litint]}.into()).unwrap();
    // Create the expression used to retrieve the element of the tensor content
    let mut list_indices = syn::parse::<ExprIndex>(quote! {self.content[index[0]]}.into()).unwrap();
    for i in 1..rank {
        list_indices = syn::parse::<ExprIndex>(quote! {#list_indices[index[#i]]}.into()).unwrap();
    }

    let dims_list_type = syn::parse::<TypeArray>(quote! {[usize; #rank_litint]}.into()).unwrap();
    let mut dims_list = syn::parse::<ExprArray>(quote! {[]}.into()).unwrap();
    for ident in dim_idents {
        let expr_path = syn::parse::<Expr>(quote! {#ident}.into()).unwrap();
        dims_list.elems.push(expr_path);
    }

    let tensor_type_ident = Ident::new(&format!("Tensor{}", rank), proc_macro2::Span::call_site());

    quote! {
        pub struct #tensor_type_ident #generics {
            pub content: #content_type
        }

        impl #generics #tensor_type_ident #generic_bind {
            const DIMS: #dims_list_type = #dims_list;
            pub fn dims(&self) -> #dims_list_type {
                Self::DIMS
            }
        }

        impl #generics Default for #tensor_type_ident #generic_bind {
            fn default() -> Self {
                Self {
                    content: #content
                }
            }
        }

        impl #generics Index<#list_index_type> for #tensor_type_ident #generic_bind {
            type Output = T;
            fn index(&self, index: #list_index_type) -> &Self::Output {
                & #list_indices
            }
        }

        impl #generics std::ops::IndexMut<#list_index_type> for #tensor_type_ident #generic_bind {
            fn index_mut(&mut self, index: #list_index_type) -> &mut Self::Output {
                &mut #list_indices
            }
        }
    }.into()
}

#[proc_macro]
pub fn test_macro(input: TokenStream) -> TokenStream {
    let a: TokenStream = quote! {
        tensor[a, 1]
    }.into();
    let parse_res = syn::parse::<syn::ExprPath>(input).unwrap();
    // let parse_res = syn::parse::<TensorComp<2, i32>>(input).unwrap();
    // let list = cleaned_indices(parse_res);
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
