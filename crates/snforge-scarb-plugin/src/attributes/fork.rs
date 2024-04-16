use super::AttributeDebugInfo;
use crate::{
    args::Arguments,
    attributes::{AttributeCollector, AttributeConsts},
    config_fn::ConfigFn,
    MacroResult,
};
use cairo_lang_macro::{Diagnostics, TokenStream};
use cairo_lang_syntax::node::db::SyntaxGroup;

pub struct ForkCollector;

impl AttributeConsts for ForkCollector {
    const ATTR_NAME: &'static str = "fork";
    const RETURN_TYPE: &'static str = "ForkConfig";
}

impl AttributeDebugInfo for ForkCollector {
    const ARGS_FORM: &'static str = "<url>: `ByteArray`, (<block_hash>: `felt252` | <block_number>: `felt252` | <block_tag>: latest)";
}

#[derive(Debug, Clone, Copy)]
enum BlockId {
    Hash,
    Number,
    Tag,
}

impl From<BlockId> for &str {
    fn from(value: BlockId) -> Self {
        match value {
            BlockId::Hash => "block_hash",
            BlockId::Number => "block_number",
            BlockId::Tag => "block_tag",
        }
    }
}
impl BlockId {
    fn as_str(self) -> &'static str {
        self.into()
    }
}

impl AttributeCollector for ForkCollector {
    fn args_into_body(db: &dyn SyntaxGroup, args: Arguments<Self>) -> Result<String, Diagnostics> {
        let args = args.named_only()?;

        let (block_id, block_args) =
            args.one_of_once(&[BlockId::Hash, BlockId::Number, BlockId::Tag])?;

        let url = args.as_once("url")?;
        let url = validate::url(db, url)?;

        let block_id_value = validate::block_id(db, block_id, block_args)?;

        let block_id_value = match block_id {
            BlockId::Hash => format!("BlockHash({block_id_value})"),
            BlockId::Number => format!("BlockNumber({block_id_value})"),
            BlockId::Tag => format!("BlockTag"),
        };

        Ok(format!("url: {url}, block: {block_id_value}"))
    }
}

pub fn _fork(args: TokenStream, item: TokenStream) -> MacroResult {
    ForkCollector::extend_with_config_fn(args, item)
}

mod validate {
    use super::BlockId;
    use cairo_lang_macro::Diagnostic;
    use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup, helpers::GetIdentifier};
    use url::Url;

    pub fn url(db: &dyn SyntaxGroup, url: &Expr) -> Result<String, Diagnostic> {
        match url {
            Expr::String(string) => match string.string_value(db) {
                None => Err(Diagnostic::error(format!("<url> is not a valid string"))),
                Some(url) => match Url::parse(&url) {
                    Ok(_) => Ok(url),
                    Err(_) => Err(Diagnostic::error(format!("<url> is not a valid url"))),
                },
            },
            _ => Err(Diagnostic::error(format!(
                "<url> invalid type, should be: double quotted string"
            ))),
        }
    }

    pub fn block_id(
        db: &dyn SyntaxGroup,
        block_id: BlockId,
        block_args: &Expr,
    ) -> Result<String, Diagnostic> {
        match block_id {
            BlockId::Tag => match block_args {
                Expr::Path(path) => {
                    let segments = path.elements(db);

                    if segments.len() == 1 {
                        let segment = segments.last().unwrap();

                        match segment.identifier(db).as_str() {
                            "latest" => Ok(String::new()),
                            _ => Err(Diagnostic::error(format!(
                                "{} value incorrect, expected: latest",
                                BlockId::Tag.as_str()
                            ))),
                        }
                    } else {
                        Err(Diagnostic::error(format!(
                            "{} value incorrect, expected: latest",
                            BlockId::Tag.as_str(),
                        )))
                    }
                }
                _ => Err(Diagnostic::error(format!(
                    "{} value incorrect, expected: latest",
                    BlockId::Tag.as_str(),
                ))),
            },
            BlockId::Hash | BlockId::Number => match block_args {
                Expr::Literal(literal) => {
                    let num = literal
                        .numeric_value(db)
                        .ok_or_else(|| Diagnostic::error("invalid number literal"))?
                        .to_str_radix(16);

                    Ok(format!("0x{num}"))
                }
                _ => Err(Diagnostic::error(format!(
                    "{} should be number literal",
                    block_id.as_str()
                ))),
            },
        }
    }
}
