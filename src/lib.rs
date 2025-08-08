use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{ToTokens, quote};
use syn::{self, Data, DeriveInput, Ident, parse_macro_input};

enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

fn parse(ast: DeriveInput) -> (Ident, Vec<Box<dyn ToTokens>>) {
    let name = ast.ident;
    let struct_fields = if let Data::Struct(struct_data) = ast.data {
        struct_data.fields
    } else {
        panic!("Expected a struct!")
    };

    let mut field_names: Vec<Box<dyn ToTokens>> = vec![];

    for (id, field) in struct_fields.into_iter().enumerate() {
        if let Some(name) = field.ident {
            field_names.push(Box::new(name));
        } else {
            // Tuple struct
            field_names.push(Box::new(Literal::usize_unsuffixed(id)))
        }
    }

    (name, field_names)
}

#[proc_macro_derive(AutoAdd)]
pub fn derive_add(input: TokenStream) -> TokenStream {
    complete(input, Ops::Add)
}

#[proc_macro_derive(AutoSub)]
pub fn derive_sub(input: TokenStream) -> TokenStream {
    complete(input, Ops::Sub)
}

#[proc_macro_derive(AutoMul)]
pub fn derive_mul(input: TokenStream) -> TokenStream {
    complete(input, Ops::Mul)
}

#[proc_macro_derive(AutoDiv)]
pub fn derive_div(input: TokenStream) -> TokenStream {
    complete(input, Ops::Div)
}

#[proc_macro_derive(AutoAddAssign)]
pub fn derive_add_assign(input: TokenStream) -> TokenStream {
    complete(input, Ops::AddAssign)
}

#[proc_macro_derive(AutoSubAssign)]
pub fn derive_sub_assign(input: TokenStream) -> TokenStream {
    complete(input, Ops::SubAssign)
}

#[proc_macro_derive(AutoMulAssign)]
pub fn derive_mul_assign(input: TokenStream) -> TokenStream {
    complete(input, Ops::MulAssign)
}

#[proc_macro_derive(AutoDivAssign)]
pub fn derive_div_assign(input: TokenStream) -> TokenStream {
    complete(input, Ops::DivAssign)
}

#[proc_macro_derive(AutoNeg)]
pub fn derive_neg(input: TokenStream) -> TokenStream {
    let (name, fields) = parse(parse_macro_input!(input as DeriveInput));

    let ret = quote! {
        impl ::std::ops::Neg for #name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                #name {
                    #(
                        #fields: -self.#fields,
                    )*
                }
            }
        }
    };
    ret.into()
}

#[proc_macro_derive(AutoAll)]
pub fn derive_all(input: TokenStream) -> TokenStream {
    let (name, fields) = parse(parse_macro_input!(input as DeriveInput));

    let add: proc_macro2::TokenStream = complete_internal(&name, &fields, Ops::Add).into();
    let sub: proc_macro2::TokenStream = complete_internal(&name, &fields, Ops::Sub).into();
    let mul: proc_macro2::TokenStream = complete_internal(&name, &fields, Ops::Mul).into();
    let div: proc_macro2::TokenStream = complete_internal(&name, &fields, Ops::Div).into();
    let add_assign: proc_macro2::TokenStream =
        complete_internal(&name, &fields, Ops::AddAssign).into();
    let sub_assign: proc_macro2::TokenStream =
        complete_internal(&name, &fields, Ops::SubAssign).into();
    let mul_assign: proc_macro2::TokenStream =
        complete_internal(&name, &fields, Ops::MulAssign).into();
    let div_assign: proc_macro2::TokenStream =
        complete_internal(&name, &fields, Ops::DivAssign).into();
    let ret = quote! {
        #add
        #sub
        #mul
        #div
        #add_assign
        #sub_assign
        #mul_assign
        #div_assign

        impl ::std::ops::Neg for #name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                #name {
                    #(
                        #fields: -self.#fields,
                    )*
                }
            }
        }
    };
    ret.into()
}

fn complete(input: TokenStream, trait_: Ops) -> TokenStream {
    let (name, fields) = parse(parse_macro_input!(input as DeriveInput));
    complete_internal(&name, &fields, trait_)
}

fn complete_internal(name: &Ident, fields: &[Box<dyn ToTokens>], trait_: Ops) -> TokenStream {
    let (trait_name, func_header, operation) = match trait_ {
        Ops::Add => (
            quote! {Add},
            quote! {add(self, rhs: T) -> Self::Output},
            quote! {+},
        ),
        Ops::Sub => (
            quote! {Sub},
            quote! {sub(self, rhs: T) -> Self::Output},
            quote! {-},
        ),
        Ops::Mul => (
            quote! {Mul},
            quote! {mul(self, rhs: T) -> Self::Output},
            quote! {*},
        ),
        Ops::Div => (
            quote! {Div},
            quote! {div(self, rhs: T) -> Self::Output},
            quote! {/},
        ),
        Ops::AddAssign => (
            quote! {AddAssign},
            quote! {add_assign(&mut self, rhs: T)},
            quote! {+},
        ),
        Ops::SubAssign => (
            quote! {SubAssign},
            quote! {sub_assign(&mut self, rhs: T)},
            quote! {-},
        ),
        Ops::MulAssign => (
            quote! {MulAssign},
            quote! {mul_assign(&mut self, rhs: T)},
            quote! {*},
        ),
        Ops::DivAssign => (
            quote! {DivAssign},
            quote! {div_assign(&mut self, rhs: T)},
            quote! {/},
        ),
    };

    let output = match trait_ {
        Ops::Add | Ops::Sub | Ops::Mul | Ops::Div => quote! {type Output = Self;},
        _ => quote! {},
    };

    let deref = match trait_ {
        Ops::AddAssign | Ops::SubAssign | Ops::MulAssign | Ops::DivAssign => quote! {*self = },
        _ => quote! {},
    };

    let ret = quote! {
        impl<T: Into<#name>> ::std::ops::#trait_name<T> for #name {
            #output

            fn #func_header {
                let rhs = rhs.into();

                #deref #name {
                    #(
                        #fields: self.#fields #operation rhs.#fields,
                    )*
                }
            }
        }
    };
    ret.into()
}
