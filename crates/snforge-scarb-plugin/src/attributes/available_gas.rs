use super::AttributeDebugInfo;
use crate::{
    args::Arguments,
    attributes::{AttributeCollector, AttributeConsts},
    config_fn::ConfigFn,
    MacroResult,
};
use cairo_lang_macro::{Diagnostic, Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup, Terminal};

pub struct AvailableGasCollector;

impl AttributeConsts for AvailableGasCollector {
    const ATTR_NAME: &'static str = "available_gas";
    const RETURN_TYPE: &'static str = "AvailableGasConfig";
}

impl AttributeDebugInfo for AvailableGasCollector {
    const ARGS_FORM: &'static str = "<usize>";
}

impl AttributeCollector for AvailableGasCollector {
    fn args_into_body(db: &dyn SyntaxGroup, args: Arguments<Self>) -> Result<String, Diagnostics> {
        let [arg] = args.unnamed_only()?.of_length::<1>()?;

        let gas: u64 = match arg {
            Expr::Literal(literal) => match literal.text(db).parse() {
                Ok(gas) => gas,
                Err(_) => Err(Diagnostic::error(format!(
                    "#[{}] argument should be number",
                    AvailableGasCollector::ATTR_NAME
                )))?,
            },
            _ => Err(Diagnostic::error(format!(
                "#[{}] argument should be number",
                AvailableGasCollector::ATTR_NAME
            )))?,
        };

        Ok(format!("gas: {gas}"))
    }
}

pub fn _available_gas(args: TokenStream, item: TokenStream) -> MacroResult {
    AvailableGasCollector::extend_with_config_fn(args, item)
}
