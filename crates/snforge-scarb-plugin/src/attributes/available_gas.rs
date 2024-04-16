use crate::{
    config_fn::{AttributeCollector, ConfigFn},
    MacroResult,
};
use cairo_lang_macro::{Diagnostic, Diagnostics, TokenStream};
use cairo_lang_syntax::node::{
    ast::{ArgClause, Expr, OptionArgListParenthesized},
    db::SyntaxGroup,
    Terminal,
};

pub struct AvailableGasCollector;

impl AttributeCollector for AvailableGasCollector {
    const ATTR_NAME: &'static str = "available_gas";
    const RETURN_TYPE: &'static str = "AvailableGasConfig";

    fn args_into_body(
        db: &dyn SyntaxGroup,
        args: OptionArgListParenthesized,
    ) -> Result<String, Diagnostics> {
        let expr =  match args {
            OptionArgListParenthesized::Empty(_) => {
                Err(Diagnostic::error(format!(
                    "#[{}] can not be used without argument. Remove it or specify argument in form: <usize>", AvailableGasCollector::ATTR_NAME
                )))
            }
            OptionArgListParenthesized::ArgListParenthesized(args) => {
                match args.arguments(db).elements(db).as_slice() {
                    [] => Err(Diagnostic::error(format!(
                        "#[{}] can not be used without argument. Remove it or specify argument in form: <usize>", AvailableGasCollector::ATTR_NAME
                    ))),
                    [arg] => {
                        match arg.arg_clause(db) {
                            ArgClause::Unnamed(value)=>Ok(value.value(db)),
                            _ => Err(Diagnostic::error(format!(
                                    "#[{}] should be used with one unnamed argument", AvailableGasCollector::ATTR_NAME
                            ))),
                        }
                    }
                    _ => Err(Diagnostic::error(format!(
                        "#[{}] should be used with one unnamed argument", AvailableGasCollector::ATTR_NAME
                    ))),
                }
            }
        }?;
        let gas: u64 = match expr {
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
