use crate::parse::parse_args;
use cairo_lang_macro::Diagnostic;
use cairo_lang_syntax::node::ast::OptionArgListParenthesized;
use cairo_lang_syntax::node::{ast::FunctionWithBody, db::SyntaxGroup, helpers::QueryAttrs};

pub mod available_gas;
pub mod fork;
pub mod fuzzer;
pub mod ignore;
pub mod should_panic;
pub mod test;

pub fn assert_is_used_once(
    db: &dyn SyntaxGroup,
    func: &FunctionWithBody,
    attr: &str,
) -> Result<(), Diagnostic> {
    if func.attributes(db).query_attr(db, attr).is_empty() {
        Ok(())
    } else {
        Err(Diagnostic::error(format!(
            "#[{attr}] can only be used once per item"
        )))
    }
}

pub fn assert_is_empty(attr: &str, args: &str) -> Result<(), Diagnostic> {
    let (_, args) = parse_args(args)?;

    match args {
        OptionArgListParenthesized::ArgListParenthesized(_) => Err(Diagnostic::warn(format!(
            "#[{attr}] does not accept any arguments"
        )))?,
        OptionArgListParenthesized::Empty(_) => Ok(()),
    }
}
