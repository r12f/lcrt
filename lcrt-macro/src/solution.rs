use crate::common::*;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Error, ItemImpl, Meta, NestedMeta, Type};

pub(crate) struct SolutionSyn {
    // ItemImpl AST
    impl_syn: ItemImpl,

    impl_ident: Ident,

    // Metadata of solution attribute
    meta: SolutionImplMeta,
}

#[derive(Debug, Clone, Default, FromMeta)]
struct SolutionImplMeta {}

impl TokenParser<SolutionSyn> for SolutionSyn {
    fn from_tokens(attr: TokenStream, item: TokenStream) -> ProcMacroResult<SolutionSyn> {
        let mut impl_syn = match syn::parse2::<ItemImpl>(item) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::new(
                    e.span(),
                    "\"solution\" attribute can only be applied to a impl block.",
                )
                .into())
            }
        };

        let impl_ident = match impl_syn.self_ty.as_ref() {
            Type::Path(p) => p.path.get_ident().unwrap().clone(),
            _ => unreachable!(
                "\"solution\" attribute can only be applied to a impl block with valid type name."
            ),
        };

        let attributes = pop_attributes(&mut impl_syn.attrs, |attr| {
            attr.to_string().as_str() == "solution"
        });

        let mut attr_metas = attributes
            .into_iter()
            .flat_map(|x| match x.parse_meta() {
                Ok(m) => {
                    let punctuated_metas = match m {
                        Meta::List(meta_list) => meta_list.nested,
                        _ => return vec![],
                    };

                    punctuated_metas
                        .into_iter()
                        .map(Ok)
                        .collect::<Vec<Result<NestedMeta, syn::Error>>>()
                }
                Err(e) => vec![Err(e)],
            })
            .collect::<Result<Vec<NestedMeta>, syn::Error>>()?;

        if !attr.is_empty() {
            let attr_metas_from_attr = match syn::parse2::<Meta>(attr) {
                Ok(v) => NestedMeta::Meta(v),
                Err(e) => {
                    return Err(Error::new(e.span(), "Parsing attribute metadata failed.").into())
                }
            };

            attr_metas.push(attr_metas_from_attr);
        }

        let parsed_meta = SolutionImplMeta::from_list(&attr_metas).unwrap();

        let syn = SolutionSyn {
            impl_syn,
            impl_ident,
            meta: parsed_meta,
        };

        Ok(syn)
    }
}

impl TokenGenerator for SolutionSyn {
    fn generate_tokens(&self) -> ProcMacroResult<TokenStream> {
        let impl_syn = &self.impl_syn;
        let impl_ident = &self.impl_ident;

        Ok(quote! {
            use lcrt::*;

            pub struct #impl_ident {}

            #impl_syn
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use syn::parse_quote;

    #[test]
    fn solution_macro_can_generate_prelude_from_attr() {
        let attr = parse_quote! {};
        let item = parse_quote! {
            impl Solution {
                pub fn add_two_numbers(
                    l1: Option<Box<ListNode>>,
                    l2: Option<Box<ListNode>>,
                ) -> Option<Box<ListNode>> {
                }
            }
        };

        let syn = SolutionSyn::from_tokens(attr, item).unwrap();
        test_utils::run_proc_macro_generation_test(
            syn,
            "test_data/solution_macro_can_generate_prelude_from_attr-expected.txt",
        );
    }

    #[test]
    fn solution_macro_can_generate_prelude_from_item() {
        let attr = parse_quote! {};
        let item = parse_quote! {
            #[solution]
            impl Solution {
                pub fn add_two_numbers(
                    l1: Option<Box<ListNode>>,
                    l2: Option<Box<ListNode>>,
                ) -> Option<Box<ListNode>> {
                }
            }
        };

        let syn = SolutionSyn::from_tokens(attr, item).unwrap();
        test_utils::run_proc_macro_generation_test(
            syn,
            "test_data/solution_macro_can_generate_prelude_from_item-expected.txt",
        );
    }

    #[test]
    fn solution_macro_can_generate_prelude_from_item_with_multiple_attributes() {
        let attr = parse_quote! {};
        let item = parse_quote! {
            #[solution]
            #[solution]
            impl Solution {
                pub fn add_two_numbers(
                    l1: Option<Box<ListNode>>,
                    l2: Option<Box<ListNode>>,
                ) -> Option<Box<ListNode>> {
                }
            }
        };

        let syn = SolutionSyn::from_tokens(attr, item).unwrap();
        test_utils::run_proc_macro_generation_test(
            syn,
            "test_data/solution_macro_can_generate_prelude_from_item_with_multiple_attributes-expected.txt",
        );
    }
}
