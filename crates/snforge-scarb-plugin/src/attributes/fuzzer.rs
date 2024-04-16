use super::AttributeDebugInfo;
use crate::{
    args::Arguments,
    attributes::{AttributeCollector, AttributeConsts},
    config_fn::ConfigFn,
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::db::SyntaxGroup;

pub struct FuzzerCollector;

impl AttributeConsts for FuzzerCollector {
    const ATTR_NAME: &'static str = "fuzzer";
    const RETURN_TYPE: &'static str = "FuzzerConfig";
}

impl AttributeDebugInfo for FuzzerCollector {
    const ARGS_FORM: &'static str = "<runs>: `u64`, <seed>: `felt252`";
}

impl AttributeCollector for FuzzerCollector {
    fn args_into_body(db: &dyn SyntaxGroup, args: Arguments<Self>) -> Result<String, Diagnostics> {
        let args = args.named_only()?;

        let seed = validate::maybe_number_value(db, args, "seed")?;
        let runs = validate::maybe_number_value(db, args, "runs")?;

        Ok(format!("seed: {seed}, runs: {runs}"))
    }
}

pub fn _fuzzer(args: TokenStream, item: TokenStream) -> MacroResult {
    FuzzerCollector::extend_with_config_fn(args, item)
}

mod validate {
    use crate::args::named::NamedArgs;
    use cairo_lang_macro::Diagnostic;
    use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup};

    pub fn maybe_number_value(
        db: &dyn SyntaxGroup,
        args: &NamedArgs,
        arg_name: &str,
    ) -> Result<String, Diagnostic> {
        let arg = args.as_once_optional(arg_name)?;

        arg.map(|arg| number(db, arg, arg_name))
            .unwrap_or(Ok("Option::None".to_string()))
    }

    pub fn number(db: &dyn SyntaxGroup, num: &Expr, arg: &str) -> Result<String, Diagnostic> {
        match num {
            Expr::Literal(literal) => {
                let num = literal
                    .numeric_value(db)
                    .ok_or_else(|| Diagnostic::error("invalid number literal"))?
                    .to_str_radix(16);

                Ok(format!("0x{num}"))
            }
            _ => Err(Diagnostic::error(
                format!("{arg} should be number literal",),
            )),
        }
    }
}
