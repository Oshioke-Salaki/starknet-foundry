use crate::args::Arguments;
use cairo_lang_macro::Diagnostics;
use cairo_lang_syntax::node::db::SyntaxGroup;

pub mod available_gas;
pub mod fork;
pub mod fuzzer;
pub mod ignore;
pub mod should_panic;
pub mod test;

pub trait AttributeConsts: Sized {
    const ATTR_NAME: &'static str;
    const RETURN_TYPE: &'static str;
}

pub trait AttributeCollector: AttributeConsts {
    fn args_into_body(db: &dyn SyntaxGroup, args: Arguments<Self>) -> Result<String, Diagnostics>;
}

pub trait AttributeDebugInfo: AttributeConsts {
    const ARGS_FORM: &'static str;
}
