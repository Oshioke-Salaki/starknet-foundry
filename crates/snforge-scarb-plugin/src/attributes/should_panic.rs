use crate::{
    config_fn::{AttributeCollector, ConfigFn},
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::OptionArgListParenthesized, db::SyntaxGroup};

pub struct ShouldPanicCollector;

impl AttributeCollector for ShouldPanicCollector {
    const ATTR_NAME: &'static str = "should_panic";
    const RETURN_TYPE: &'static str = "ShouldPanicConfig";

    fn args_into_body(
        db: &dyn SyntaxGroup,
        args: OptionArgListParenthesized,
    ) -> Result<String, Diagnostics> {
        Ok(Default::default())
    }
}

pub fn _should_panic(args: TokenStream, item: TokenStream) -> MacroResult {
    ShouldPanicCollector::extend_with_config_fn(args, item)
}
