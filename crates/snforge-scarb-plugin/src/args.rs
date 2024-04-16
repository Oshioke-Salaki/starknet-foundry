use self::{named::NamedArgs, unnamed::UnnamedArgs};
use crate::attributes::AttributeDebugInfo;
use cairo_lang_macro::Diagnostic;
use cairo_lang_syntax::node::{
    ast::{ArgClause, Expr, OptionArgListParenthesized},
    db::SyntaxGroup,
    Terminal,
};
use smol_str::SmolStr;
use std::{collections::HashMap, marker::PhantomData};

pub mod named;
pub mod unnamed;

#[derive(Debug)]
pub struct Arguments<T> {
    pub named: NamedArgs,
    pub unnamed: HashMap<usize, Expr>,
    pub shorthand: HashMap<usize, SmolStr>,
    _attr: PhantomData<T>,
}

impl<T> Default for Arguments<T> {
    fn default() -> Self {
        Self {
            named: Default::default(),
            unnamed: Default::default(),
            shorthand: Default::default(),
            _attr: Default::default(),
        }
    }
}

impl<T: AttributeDebugInfo> Arguments<T> {
    pub fn new(db: &dyn SyntaxGroup, args: OptionArgListParenthesized) -> Self {
        let args = match args {
            OptionArgListParenthesized::Empty(_) => vec![],
            OptionArgListParenthesized::ArgListParenthesized(args) => {
                args.arguments(db).elements(db)
            }
        };

        args.into_iter()
            .enumerate()
            .fold(Self::default(), |mut acc, (i, arg)| {
                match arg.arg_clause(db) {
                    ArgClause::Unnamed(value) => {
                        acc.unnamed.insert(i, value.value(db));
                    }
                    ArgClause::FieldInitShorthand(value) => {
                        acc.shorthand.insert(i, value.name(db).name(db).text(db));
                    }
                    ArgClause::Named(value) => {
                        acc.named
                            .entry(value.name(db).text(db))
                            .or_default()
                            .push(value.value(db));
                    }
                };

                acc
            })
    }

    #[inline]
    fn is_both_empty<K2, K3, V2, V3>(a: &HashMap<K2, V2>, b: &HashMap<K3, V3>) -> bool {
        a.is_empty() && b.is_empty()
    }

    #[inline]
    pub fn named_only(&self) -> Result<&NamedArgs, Diagnostic> {
        if Self::is_both_empty(&self.shorthand, &self.unnamed) {
            Ok(&self.named)
        } else {
            Err(Diagnostic::error(format!(
                "#[{}] can be used with named attributes only",
                T::ATTR_NAME
            )))
        }
    }

    #[inline]
    pub fn unnamed_only(&self) -> Result<UnnamedArgs, Diagnostic> {
        if Self::is_both_empty(&self.shorthand, &self.named) {
            Ok(UnnamedArgs::new(&self.unnamed))
        } else {
            Err(Diagnostic::error(format!(
                "#[{}] can be used with unnamed attributes only",
                T::ATTR_NAME
            )))
        }
    }
}
