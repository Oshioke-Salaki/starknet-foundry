use crate::{
    config_fn::{AttributeCollector, ConfigFn},
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::OptionArgListParenthesized, db::SyntaxGroup};
use indoc::formatdoc;

pub struct FuzzerCollector;

impl AttributeCollector for FuzzerCollector {
    const ATTR_NAME: &'static str = "fuzzer";
    const RETURN_TYPE: &'static str = "FuzzerConfig";

    fn args_into_body(
        db: &dyn SyntaxGroup,
        args: OptionArgListParenthesized,
    ) -> Result<String, Diagnostics> {
        //TODO
        Ok(formatdoc!(
            "
            
            "
        ))
    }
}

pub fn _fuzzer(args: TokenStream, item: TokenStream) -> MacroResult {
    FuzzerCollector::extend_with_config_fn(args, item)
}
