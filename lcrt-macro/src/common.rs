use proc_macro2::{Ident, TokenStream};
use syn::Attribute;

#[derive(thiserror::Error, Debug)]
pub enum ProcMacroError {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl ProcMacroError {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            ProcMacroError::Syn(err) => err.to_compile_error(),
            ProcMacroError::Darling(err) => err.write_errors(),
        }
    }
}

pub type ProcMacroResult<T> = std::result::Result<T, ProcMacroError>;

pub trait TokenParser<T: TokenGenerator> {
    fn from_tokens(attr: TokenStream, item: TokenStream) -> ProcMacroResult<T>;
}

pub trait TokenGenerator {
    fn generate_tokens(&self) -> ProcMacroResult<TokenStream>;

    fn to_tokens(&self) -> TokenStream {
        match self.generate_tokens() {
            Ok(v) => v,
            Err(err) => err.into_token_stream(),
        }
    }
}

pub fn process_attribute_macro<T: TokenParser<T> + TokenGenerator>(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    process_attribute_macro2::<T>(attr.into(), item.into()).into()
}

pub fn process_attribute_macro2<T: TokenParser<T> + TokenGenerator>(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let generator = match T::from_tokens(attr, item) {
        Ok(v) => v,
        Err(err) => return err.into_token_stream(),
    };

    generator.to_tokens()
}

pub fn pop_attributes<F: Fn(&Ident) -> bool>(
    attrs: &mut Vec<Attribute>,
    pred: F,
) -> Vec<Attribute> {
    let mut indices = attrs
        .iter()
        .enumerate()
        .filter(|(index, item)| match item.path.get_ident() {
            Some(attr_id) => pred(attr_id),
            None => false,
        })
        .map(|(index, item)| index)
        .collect::<Vec<usize>>();

    indices.reverse();
    indices
        .into_iter()
        .map(|index| attrs.swap_remove(index))
        .collect::<Vec<Attribute>>()
}
