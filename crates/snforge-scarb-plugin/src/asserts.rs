use crate::{attributes::test::TEST_ATTR_NAME, parse::parse_args};
use cairo_lang_macro::Diagnostic;
use cairo_lang_syntax::node::{
    ast::{FunctionWithBody, OptionArgListParenthesized},
    db::SyntaxGroup,
    helpers::QueryAttrs,
};

pub fn assert_is_used_once(
    db: &dyn SyntaxGroup,
    func: &FunctionWithBody,
    attr: &str,
) -> Result<(), Diagnostic> {
    if !func.attributes(db).has_attr(db, attr) {
        Ok(())
    } else {
        Err(Diagnostic::error(format!(
            "#[{attr}] can only be used once per item"
        )))
    }
}

pub fn assert_is_used_on_test(
    db: &dyn SyntaxGroup,
    func: &FunctionWithBody,
    attr: &str,
) -> Result<(), Diagnostic> {
    if func.attributes(db).has_attr(db, TEST_ATTR_NAME) {
        Ok(())
    } else {
        Err(Diagnostic::error(format!(
            "#[{attr}] can only be used on functions marked with #[{TEST_ATTR_NAME}]"
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
