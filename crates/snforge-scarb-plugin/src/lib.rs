use cairo_lang_macro::{
    attribute_macro, post_process, AuxData, PostProcessContext, ProcMacroResult, TokenStream,
};

#[attribute_macro]
pub fn test(token_stream: TokenStream) -> ProcMacroResult {
    ProcMacroResult {
        token_stream,
        aux_data: Some(AuxData::new(vec![])),
        diagnostics: Default::default(),
        full_path_markers: Default::default(),
    }
}
#[attribute_macro]
pub fn ignore(token_stream: TokenStream) -> ProcMacroResult {
    ProcMacroResult {
        token_stream,
        aux_data: Some(AuxData::new(vec![])),
        diagnostics: Default::default(),
        full_path_markers: Default::default(),
    }
}
#[attribute_macro]
pub fn fork(token_stream: TokenStream) -> ProcMacroResult {
    ProcMacroResult {
        token_stream,
        aux_data: Some(AuxData::new(vec![])),
        diagnostics: Default::default(),
        full_path_markers: Default::default(),
    }
}
#[attribute_macro]
pub fn fuzzer(token_stream: TokenStream) -> ProcMacroResult {
    ProcMacroResult {
        token_stream,
        aux_data: Some(AuxData::new(vec![])),
        diagnostics: Default::default(),
        full_path_markers: Default::default(),
    }
}
#[attribute_macro]
pub fn available_gas(token_stream: TokenStream) -> ProcMacroResult {
    ProcMacroResult {
        token_stream,
        aux_data: Some(AuxData::new(vec![])),
        diagnostics: Default::default(),
        full_path_markers: Default::default(),
    }
}

#[post_process]
pub fn post_hook(_context: PostProcessContext) {}
