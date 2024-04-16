use super::AttributeDebugInfo;
use crate::{
    args::Arguments,
    asserts::assert_is_empty,
    attributes::{AttributeCollector, AttributeConsts},
    config_fn::ConfigFn,
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::db::SyntaxGroup;

pub struct IgnoreCollector;

impl AttributeConsts for IgnoreCollector {
    const ATTR_NAME: &'static str = "ignore";
    const RETURN_TYPE: &'static str = "IgnoreConfig";
}

impl AttributeDebugInfo for IgnoreCollector {
    const ARGS_FORM: &'static str = "";
}

impl AttributeCollector for IgnoreCollector {
    fn args_into_body(
        _db: &dyn SyntaxGroup,
        _args: Arguments<Self>,
    ) -> Result<String, Diagnostics> {
        Ok("is_ignored: true".to_string())
    }
}

pub fn _ignore(args: TokenStream, item: TokenStream) -> MacroResult {
    assert_is_empty(IgnoreCollector::ATTR_NAME, &args.to_string())?;

    IgnoreCollector::extend_with_config_fn(args, item)
}
