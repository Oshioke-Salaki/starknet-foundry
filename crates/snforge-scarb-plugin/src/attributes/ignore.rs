use super::assert_is_empty;
use crate::{
    config_fn::{AttributeCollector, ConfigFn},
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::OptionArgListParenthesized, db::SyntaxGroup};

pub struct IgnoreCollector;

impl AttributeCollector for IgnoreCollector {
    const ATTR_NAME: &'static str = "ignore";
    const RETURN_TYPE: &'static str = "IgnoreConfig";

    fn args_into_body(
        _db: &dyn SyntaxGroup,
        _args: OptionArgListParenthesized,
    ) -> Result<String, Diagnostics> {
        Ok("is_ignored: true".to_string())
    }
}

pub fn _ignore(args: TokenStream, item: TokenStream) -> MacroResult {
    assert_is_empty(IgnoreCollector::ATTR_NAME, &args.to_string())?;

    IgnoreCollector::extend_with_config_fn(args, item)
}
