use crate::{
    attributes::assert_is_used_once,
    parse::{parse, parse_args},
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::{ast::OptionArgListParenthesized, db::SyntaxGroup, Terminal};
use cairo_lang_utils::Upcast;
use indoc::formatdoc;

pub trait AttributeCollector {
    const ATTR_NAME: &'static str;
    const RETURN_TYPE: &'static str;

    fn args_into_body(
        db: &dyn SyntaxGroup,
        args: OptionArgListParenthesized,
    ) -> Result<String, Diagnostics>;
}

pub trait ConfigFn {
    fn get_config_fn_name(origin_fn_name: &str) -> String;
    fn create_config_fn(origin_fn_name: &str, body: &str) -> String;
    fn extend_with_config_fn(args: TokenStream, item: TokenStream) -> MacroResult;
}

impl<T> ConfigFn for T
where
    T: AttributeCollector,
{
    fn get_config_fn_name(origin_fn_name: &str) -> String {
        let attr_name = Self::ATTR_NAME;

        format!("snforge_internal_prefix_{origin_fn_name}_{attr_name}")
    }

    fn create_config_fn(origin_fn_name: &str, body: &str) -> String {
        let fn_name = Self::get_config_fn_name(origin_fn_name);
        let return_type = Self::RETURN_TYPE;

        format!("fn {fn_name}()->Option<snforge_std::_config_types::{return_type}>{{{body}}}")
    }

    fn extend_with_config_fn(args: TokenStream, item: TokenStream) -> MacroResult {
        let item = item.to_string();
        let (db, func) = parse(&item, Self::ATTR_NAME)?;

        let db = db.upcast();

        assert_is_used_once(db, &func, Self::ATTR_NAME)?;

        let (args_db, args) = parse_args(&args.to_string())?;
        let return_type = Self::RETURN_TYPE;

        let config_fn = Self::create_config_fn(
            func.declaration(db).name(db).text(db).as_str(),
            &format!(
                "Option::Some(snforge_std::_config_types::{return_type}{{{}}})",
                Self::args_into_body(args_db.upcast(), args)?
            ),
        );

        Ok(TokenStream::new(formatdoc!(
            "
                {config_fn}
                {item}
            "
        )))
    }
}
