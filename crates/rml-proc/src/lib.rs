#![feature(box_patterns)]
#![feature(extract_if)]

use func::fn_spec_item;
use proc_macro::TokenStream as TS1;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{quote, quote_spanned};
use rml_syn::{subject::LogicSubject, Encode, Spec, TBlock, Term};

use syn::{parse_macro_input, parse_quote, spanned::Spanned, ReturnType};

mod func;
mod loop_inv;
mod subject;
mod util;

use subject::{ContractSubject, InvariantSubject};
use util::generate_unique_ident;

#[proc_macro_attribute]
pub fn spec(attr: TS1, item: TS1) -> TS1 {
    let sp = parse_macro_input!(attr as Spec);

    let item = parse_macro_input!(item as ContractSubject);

    let spec_name = generate_unique_ident(&item.name());
    let name_tag = format!("{}", quote! { #spec_name });

    match item {
        ContractSubject::FnOrMethod(fn_or_meth) if fn_or_meth.is_trait_signature() => {
            TS1::from(quote! {
                #fn_or_meth
            })
        }
        ContractSubject::FnOrMethod(fn_or_meth) => {
            let result = match fn_or_meth.sig.output {
                ReturnType::Default => parse_quote! { result : () },
                ReturnType::Type(_, ref ty) => parse_quote! { result : #ty },
            };
            let spec_tokens = fn_spec_item(
                spec_name,
                fn_or_meth.sig.clone(),
                result,
                sp,
                Span::call_site(),
            );
            TS1::from(quote! {
                #spec_tokens
                #[rml::spec_case_ref=#name_tag]
                #fn_or_meth
            })
        }
        ContractSubject::Closure(c) => TS1::from(quote! {
            #c
        }),
    }
}

#[proc_macro_attribute]
pub fn strictly_pure(attr: TS1, item: TS1) -> TS1 {
    assert!(attr.is_empty(), "`strictly_pure` takes no arguments");
    let toks = TS2::from(item);
    TS1::from(quote! {
        #[rml::decl::strictly_pure]
        #toks
    })
}

#[proc_macro_attribute]
pub fn pure(attr: TS1, item: TS1) -> TS1 {
    assert!(attr.is_empty(), "`pure` takes no arguments");
    let toks = TS2::from(item);
    TS1::from(quote! {
        #[rml::decl::pure]
        #toks
    })
}

#[proc_macro_attribute]
pub fn invariant(attr: TS1, item: TS1) -> TS1 {
    let term = parse_macro_input!(attr as Term);
    let subject = parse_macro_input!(item as InvariantSubject);
    let sp = subject.span();
    let ts = match subject {
        InvariantSubject::Loop(l) => {
            let (attrs, stmts, l) = match loop_inv::loop_inv(term, l) {
                Ok(r) => r,
                Err(err) => return TS1::from(err.to_compile_error()),
            };
            quote_spanned! { sp=>
                {
                    #stmts
                    #attrs
                    #l
                }
            }
        }
        InvariantSubject::Item(_) => todo!(),
    };

    TS1::from(ts)
}

#[proc_macro_attribute]
pub fn variant(_attr: TS1, item: TS1) -> TS1 {
    item
}

#[proc_macro_attribute]
pub fn modifies(_attr: TS1, item: TS1) -> TS1 {
    item
}

#[proc_macro_attribute]
pub fn logic(attr: TS1, item: TS1) -> TS1 {
    assert!(attr.is_empty(), "`logic` takes no arguments");
    let subject = parse_macro_input!(item as LogicSubject);
    match subject {
        LogicSubject::WithBody(f) => {
            let sp = f.span();
            TS1::from(quote_spanned! { sp =>
                #[rml::decl::logic]
                #f
            })
        }
        LogicSubject::WithoutBody(t) => {
            let sp = t.span();
            TS1::from(quote_spanned! { sp =>
                #[rml::decl::logic]
                #t
            })
        }
    }
}

#[proc_macro_attribute]
pub fn trusted(attr: TS1, item: TS1) -> TS1 {
    assert!(attr.is_empty(), "`trusted` takes no arguments");
    let toks = TS2::from(item);
    TS1::from(quote! {
        #[rml::delc::trusted]
        #toks
    })
}

#[proc_macro]
pub fn rml(tokens: TS1) -> TS1 {
    let block = parse_macro_input!(tokens with TBlock::parse_within);
    TS1::from(
        block
            .into_iter()
            .map(|ts| {
                let sp = ts.span();
                let stmt = ts.encode();
                quote_spanned! { sp => #stmt }
            })
            .collect::<TS2>(),
    )
}

#[proc_macro]
pub fn proof_assert(assertion: TS1) -> TS1 {
    let assertion = parse_macro_input!(assertion with TBlock::parse_within);
    let body = assertion
        .into_iter()
        .map(|ts| {
            let sp = ts.span();
            let stmt = ts.encode();
            quote_spanned! { sp => #stmt }
        })
        .collect::<TS2>();
    TS1::from(quote! {
        {
            #[allow(unused_must_use, unused_variables)]
            let _ = {
                #[rml::spec::assert]
                || {
                    let b: bool = {
                        #body
                    }
                    b
                }
            }
        }
    })
}
