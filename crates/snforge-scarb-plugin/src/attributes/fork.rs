use crate::{
    config_fn::{AttributeCollector, ConfigFn},
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::OptionArgListParenthesized, db::SyntaxGroup};
use indoc::formatdoc;

pub struct ForkCollector;

impl AttributeCollector for ForkCollector {
    const ATTR_NAME: &'static str = "fork";
    const RETURN_TYPE: &'static str = "ForkConfig";

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

pub fn _fork(args: TokenStream, item: TokenStream) -> MacroResult {
    ForkCollector::extend_with_config_fn(args, item)
}
