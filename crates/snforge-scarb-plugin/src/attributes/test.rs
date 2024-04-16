use crate::{
    asserts::{assert_is_empty, assert_is_used_once},
    attributes::{
        available_gas::AvailableGasCollector, fork::ForkCollector, fuzzer::FuzzerCollector,
        ignore::IgnoreCollector, should_panic::ShouldPanicCollector, AttributeCollector,
        AttributeDebugInfo,
    },
    config_fn::ConfigFn,
    parse::parse,
    MacroResult,
};
use cairo_lang_macro::TokenStream;
use cairo_lang_syntax::node::{ast::FunctionWithBody, db::SyntaxGroup, helpers::QueryAttrs, Token};
use cairo_lang_utils::Upcast;
use indoc::formatdoc;

pub const TEST_ATTR_NAME: &str = "test";

pub fn _test(args: TokenStream, item: TokenStream) -> MacroResult {
    let code = item.to_string();
    let (simple_db, func) = parse(&code, TEST_ATTR_NAME)?;
    let db = simple_db.upcast();

    assert_is_used_once(db, &func, TEST_ATTR_NAME)?;
    assert_is_empty(TEST_ATTR_NAME, &args.to_string())?;

    Ok(TokenStream::new(format!(
        "{} {} {code}",
        default_config_functions(&func, db),
        config_fn(func.declaration(db).name(db).token(db).text(db).as_str())
    )))
}

fn config_fn(origin_fn_name: &str) -> String {
    let gas = AvailableGasCollector::get_config_fn_name(origin_fn_name);
    let fork = ForkCollector::get_config_fn_name(origin_fn_name);
    let should_panic = ShouldPanicCollector::get_config_fn_name(origin_fn_name);
    let fuzzer = FuzzerCollector::get_config_fn_name(origin_fn_name);
    let ignore = IgnoreCollector::get_config_fn_name(origin_fn_name);

    formatdoc!(
        "
        {origin_fn_name}__config__snforge() -> TestConfig {{
            TestConfig {{
                gas: {gas}(),
                fork: {fork}(),
                fuzzer: {fuzzer}(),
                should_panic: {should_panic}(),
                ignore: {ignore}(),
            }}
        }}
        "
    )
}

/// if there was no configuration attribute create it's config function that returns `None`
fn default_config_functions(func: &FunctionWithBody, db: &dyn SyntaxGroup) -> String {
    fn default_fn<T: AttributeCollector + AttributeDebugInfo>(
        func: &FunctionWithBody,
        db: &dyn SyntaxGroup,
    ) -> Option<String> {
        if func.attributes(db).has_attr(db, T::ATTR_NAME) {
            None
        } else {
            let original_fn_name = func.declaration(db).name(db).token(db).text(db);

            Some(T::create_config_fn(
                original_fn_name.as_str(),
                "Option::None",
            ))
        }
    }

    [
        default_fn::<AvailableGasCollector>(func, db),
        default_fn::<ForkCollector>(func, db),
        default_fn::<ShouldPanicCollector>(func, db),
        default_fn::<FuzzerCollector>(func, db),
        default_fn::<IgnoreCollector>(func, db),
    ]
    .into_iter()
    .filter_map(|a| a)
    .fold(String::new(), |acc, default_fn_code| acc + &default_fn_code)
}
