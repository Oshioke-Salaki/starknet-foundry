use cairo_lang_macro::Diagnostic;
use cairo_lang_syntax::node::ast::Expr;
use std::{collections::HashMap, ops::Deref};

pub struct UnnamedArgs<'a>(Vec<&'a Expr>);

impl<'a> Deref for UnnamedArgs<'a> {
    type Target = Vec<&'a Expr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UnnamedArgs<'_> {
    pub fn new<'a>(unnamed: &'a HashMap<usize, Expr>) -> UnnamedArgs<'a> {
        let mut args: Vec<_> = unnamed.iter().collect();

        args.sort_by(|(a, _), (b, _)| a.cmp(b));

        let args = args.into_iter().map(|(_, expr)| expr).collect();

        UnnamedArgs(args)
    }
}

impl<'a> UnnamedArgs<'a> {
    pub fn of_length<const T: usize>(&self) -> Result<[&'a Expr; T], Diagnostic> {
        if self.len() != T {
            Err(Diagnostic::error(format!(
                "expected {} arguments, got: {}",
                T,
                self.len()
            )))
        } else {
            let result: [&'a Expr; T] = self
                .as_slice()
                .try_into()
                //checked if self.len() == T
                .unwrap();

            Ok(result)
        }
    }
}
