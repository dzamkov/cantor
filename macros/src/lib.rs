use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::*;

#[proc_macro_derive(Finite)]
pub fn derive_finite(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (count, index_of, nth) = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let mut field_tys = Vec::new();
                let mut field_idents = Vec::new();
                for field in fields.named {
                    field_tys.push(field.ty.to_token_stream());
                    field_idents.push(field.ident.to_token_stream());
                }
                let count = product_count(&field_tys);
                let index_of = product_index_of(&field_tys, &field_idents);
                let nth = product_nth(
                    &field_tys,
                    quote! { index },
                    &field_idents,
                    quote! { Self { #(#field_idents),* } },
                );
                (
                    quote! { #count },
                    quote! {
                        let Self { #(#field_idents),* } = value;
                        #index_of
                    },
                    quote! {
                        if index < <Self as ::cantor::Finite>::COUNT {
                            Some(#nth)
                        } else {
                            None
                        }
                    },
                )
            }
            Fields::Unnamed(fields) => {
                let mut field_tys = Vec::new();
                let mut field_idents = Vec::new();
                for field in fields.unnamed {
                    field_tys.push(field.ty.to_token_stream());
                    let field_ident = format!("f{}", field_idents.len());
                    let field_ident = Ident::new(&field_ident, Span::call_site());
                    field_idents.push(field_ident.to_token_stream());
                }
                let count = product_count(&field_tys);
                let index_of = product_index_of(&field_tys, &field_idents);
                let nth = product_nth(
                    &field_tys,
                    quote! { index },
                    &field_idents,
                    quote! { Self(#(#field_idents),*) },
                );
                (
                    quote! { #count },
                    quote! {
                        let Self(#(#field_idents),*) = value;
                        #index_of
                    },
                    quote! {
                        if index < <Self as ::cantor::Finite>::COUNT {
                            Some(#nth)
                        } else {
                            None
                        }
                    },
                )
            }
            Fields::Unit => (
                quote! { 1 },
                quote! { 0 },
                quote! {
                    if index < 1 {
                        Some(Self)
                    } else {
                        None
                    }
                },
            ),
        },
        Data::Enum(data) => {
            // Gather info from variants
            let mut count = SumExpr::new_zero();
            let mut const_count = SumExpr::new_zero();
            let mut consts = Vec::new();
            let mut index_of_arms = Vec::new();
            let mut nth_arms = Vec::new();
            for variant in data.variants {
                // Consider the different types of variant definitions
                let variant_name = variant.ident;
                let start_index = const_count.get_simple(&mut consts);
                const_count.set_zero();
                const_count.add(start_index.clone().into());
                match variant.fields {
                    Fields::Named(fields) => {
                        let mut field_tys = Vec::new();
                        let mut field_idents = Vec::new();
                        for field in fields.named {
                            field_tys.push(field.ty.to_token_stream());
                            field_idents.push(field.ident.to_token_stream());
                        }
                        let index_of_arm = product_index_of(&field_tys, &field_idents);
                        index_of_arms.push(quote! {
                            Self::#variant_name { #(#field_idents),* } => #count + #index_of_arm
                        });
                        let nth_arm = product_nth(
                            &field_tys,
                            quote! { index - #start_index },
                            &field_idents,
                            quote! { Self::#variant_name { #(#field_idents),* } },
                        );
                        let variant_count = product_count(&field_tys);
                        count.add(variant_count.clone());
                        const_count.add(variant_count);
                        const_count.add(NumTerm::Literal(-1));
                        let end_index = const_count.get_simple(&mut consts);
                        const_count.set_zero();
                        const_count.add(end_index.clone().into());
                        const_count.add(NumTerm::Literal(1));
                        nth_arms.push(quote! {
                            #start_index..=#end_index => Some(#nth_arm)
                        });
                    }
                    Fields::Unnamed(fields) => {
                        let mut field_tys = Vec::new();
                        let mut field_idents = Vec::new();
                        for field in fields.unnamed {
                            field_tys.push(field.ty.to_token_stream());
                            let field_ident = format!("f{}", field_idents.len());
                            let field_ident = Ident::new(&field_ident, Span::call_site());
                            field_idents.push(field_ident.to_token_stream());
                        }
                        let index_of_arm = product_index_of(&field_tys, &field_idents);
                        index_of_arms.push(quote! {
                            Self::#variant_name(#(#field_idents),*) => #count + #index_of_arm
                        });
                        let nth_arm = product_nth(
                            &field_tys,
                            quote! { index - #start_index },
                            &field_idents,
                            quote! { Self::#variant_name(#(#field_idents),*) },
                        );
                        let variant_count = product_count(&field_tys);
                        count.add(variant_count.clone());
                        const_count.add(variant_count);
                        const_count.add(NumTerm::Literal(-1));
                        let end_index = const_count.get_simple(&mut consts);
                        const_count.set_zero();
                        const_count.add(end_index.clone().into());
                        const_count.add(NumTerm::Literal(1));
                        nth_arms.push(quote! {
                            #start_index..=#end_index => Some(#nth_arm)
                        });
                    }
                    Fields::Unit => {
                        index_of_arms.push(quote! {
                            Self::#variant_name => #start_index
                        });
                        nth_arms.push(quote! {
                            #start_index => Some(Self::#variant_name)
                        });
                        count.add(NumTerm::Literal(1));
                        const_count.add(NumTerm::Literal(1));
                    }
                };
            }
            nth_arms.push(quote! { _ => None });
            (
                quote! { #count },
                quote! {
                    #(#consts)*
                    match value {
                        #(#index_of_arms,)*
                    }
                },
                quote! {
                    #(#consts)*
                    match index {
                        #(#nth_arms,)*
                    }
                },
            )
        }
        Data::Union(_) => todo!(),
    };

    // Build implementation
    let mut res = quote! {
        #[automatically_derived]
        unsafe impl #impl_generics ::cantor::Finite for #name #ty_generics #where_clause {
            const COUNT: usize = #count;

            fn index_of(value: Self) -> usize {
                #index_of
            }

            fn nth(index: usize) -> Option<Self> {
                #nth
            }
        }
    };

    // If this is a concrete type (no generic parameters), also implement helper traits.
    if input.generics.type_params().next().is_none() {
        res.extend(quote! {
            ::cantor::impl_concrete_finite!(#name);
        });
    }

    // Return final result
    TokenStream::from(res)
}

/// A [`NumTerm`] that can be used as a range bound.
#[derive(Clone)]
enum SimpleNumTerm {
    Literal(i64),
    Constant(Ident),
}

impl ToTokens for SimpleNumTerm {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SimpleNumTerm::Literal(value) => {
                tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(*value)))
            }
            SimpleNumTerm::Constant(ident) => tokens.append(TokenTree::Ident(ident.clone())),
        }
    }
}

/// A [`NumTerm`] which is not a literal.
enum NonLiteralNumTerm {
    Constant(Ident),
    Complex(TokenStream2),
}

impl ToTokens for NonLiteralNumTerm {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            NonLiteralNumTerm::Constant(ident) => tokens.append(TokenTree::Ident(ident.clone())),
            NonLiteralNumTerm::Complex(expr) => tokens.extend(expr.clone()),
        }
    }
}

/// A term which provides a number.
#[derive(Clone)]
enum NumTerm {
    Literal(i64),
    Constant(Ident),
    Complex(TokenStream2),
}

impl From<SimpleNumTerm> for NumTerm {
    fn from(term: SimpleNumTerm) -> Self {
        match term {
            SimpleNumTerm::Literal(value) => NumTerm::Literal(value),
            SimpleNumTerm::Constant(ident) => NumTerm::Constant(ident),
        }
    }
}

impl ToTokens for NumTerm {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            NumTerm::Literal(value) => {
                tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(*value)))
            }
            NumTerm::Constant(ident) => tokens.append(TokenTree::Ident(ident.clone())),
            NumTerm::Complex(expr) => tokens.extend(expr.clone()),
        }
    }
}

/// An expression for a sum of values.
struct SumExpr {
    lit: i64,
    non_lit: Vec<NonLiteralNumTerm>,
}

impl SumExpr {
    /// Creates a [`SumExpr`] with an initial value of zero.
    pub fn new_zero() -> Self {
        Self {
            lit: 0,
            non_lit: Vec::new(),
        }
    }

    /// Adds a value to this expression.
    pub fn add(&mut self, value: NumTerm) {
        match value {
            NumTerm::Literal(value) => self.lit += value,
            NumTerm::Constant(value) => self.non_lit.push(NonLiteralNumTerm::Constant(value)),
            NumTerm::Complex(value) => self.non_lit.push(NonLiteralNumTerm::Complex(value)),
        }
    }

    /// Sets this expression to 0.
    pub fn set_zero(&mut self) {
        self.lit = 0;
        self.non_lit.clear();
    }

    /// Gets a [`SimpleNumTerm`] representation of this expression, assuming its possible to define
    /// an arbitrary constant ahead of time.
    pub fn get_simple(&mut self, consts: &mut Vec<TokenStream2>) -> SimpleNumTerm {
        if self.non_lit.is_empty() {
            return SimpleNumTerm::Literal(self.lit);
        } else if self.lit == 0 && self.non_lit.len() == 1 {
            if let NonLiteralNumTerm::Constant(ident) = &self.non_lit[0] {
                return SimpleNumTerm::Constant(ident.clone());
            }
        }
        let ident = format!("C_{}", consts.len());
        let ident = Ident::new(&ident, Span::call_site());
        consts.push(quote! { const #ident: usize = #self; });
        SimpleNumTerm::Constant(ident)
    }
}

impl ToTokens for SumExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if let Some((head_non_lit, tail_non_lit)) = self.non_lit.split_first() {
            if self.lit > 0 {
                tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(self.lit)));
                tokens.extend(quote! { + });
            }
            tokens.extend(quote! { #head_non_lit #(+ #tail_non_lit)* });
            if self.lit < 0 {
                tokens.extend(quote! { - });
                tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(-self.lit)));
            }
        } else {
            tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(self.lit)));
        }
    }
}

/// Gets an expression for the number of values for a product of the given types.
fn product_count(field_tys: &[TokenStream2]) -> NumTerm {
    if let Some((head_field_ty, tail_field_tys)) = field_tys.split_first() {
        NumTerm::Complex(quote! {
            <#head_field_ty as ::cantor::Finite>::COUNT
            #(* <#tail_field_tys as ::cantor::Finite>::COUNT)*
        })
    } else {
        NumTerm::Literal(1)
    }
}

/// Gets an expression which produces the index of a value of the product type, given the values
/// of its fields.
fn product_index_of(field_tys: &[TokenStream2], fields: &[TokenStream2]) -> TokenStream2 {
    quote! {
        {
            let __index = 0;
            #(let __index = __index *
                <#field_tys as ::cantor::Finite>::COUNT +
                <#field_tys as ::cantor::Finite>::index_of(#fields);)*
            __index
        }
    }
}

/// Gets an expression which produces a value of the product, given an expression for a
/// valid index and a constructor for values of the product.
fn product_nth(
    field_tys: &[TokenStream2],
    index: TokenStream2,
    fields: &[TokenStream2],
    cons: TokenStream2,
) -> TokenStream2 {
    let field_tys_rev = field_tys.iter().rev();
    let fields_rev = fields.iter().rev();
    quote! {
        {
            let __index = #index;
            #(
                let #fields_rev = <#field_tys_rev as ::cantor::Finite>::nth(__index %
                    <#field_tys_rev as ::cantor::Finite>::COUNT).unwrap();
                let __index = __index / <#field_tys_rev as ::cantor::Finite>::COUNT;
            )*
            #cons
        }
    }
}
