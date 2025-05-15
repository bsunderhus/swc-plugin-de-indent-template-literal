use crate::*;
use serde::Deserialize;
use swc_core::{
    common::comments::{CommentKind, Comments},
    ecma::{ast::Tpl, visit::VisitMut},
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeIndentVisitorConfig {
    #[serde(default = "DeIndentVisitorConfig::default_tag")]
    pub tag: String,
    #[serde(default)]
    pub indent_style: IndentStyle,
}

impl DeIndentVisitorConfig {
    fn default_tag() -> String {
        "#__DE-INDENT__".to_string()
    }
}

pub struct DeIndentVisitor<C>
where
    C: Comments,
{
    comments: C,
    tag: String,
    indent_style: IndentStyle,
}

impl<C> DeIndentVisitor<C>
where
    C: Comments,
{
    pub fn new(comments: C, config: DeIndentVisitorConfig) -> Self {
        Self {
            comments,
            tag: config.tag,
            indent_style: config.indent_style,
        }
    }
}

impl<C> VisitMut for DeIndentVisitor<C>
where
    C: Comments,
{
    fn visit_mut_tpl(&mut self, tpl: &mut Tpl) {
        tpl.visit_mut_children_with(self);

        if self.comments.has_leading(tpl.span.lo()) {
            let mut comments = self
                .comments
                .take_leading(tpl.span.lo())
                .unwrap_or_default();

            let extracted_comments = comments
                .extract_if(.., |comment| {
                    comment.kind == CommentKind::Block
                        && comment
                            .text
                            .to_string()
                            .trim_matches(|char: char| char.is_whitespace() || char == '*')
                            == &self.tag
                })
                .collect::<Vec<_>>();

            self.comments.add_leading_comments(tpl.span.lo(), comments);

            let magic_string = "$$--JOIN_QUASI--$$";

            if extracted_comments.len() > 0 {
                tpl.quasis
                    .iter()
                    .map(|quasi| quasi.raw.to_string())
                    .collect::<Vec<_>>()
                    .join(magic_string)
                    .de_indent(self.indent_style)
                    .split(magic_string)
                    .enumerate()
                    .for_each(|(index, de_indented_raw)| {
                        tpl.quasis[index].raw = de_indented_raw.into();
                    });
            }
        }
    }
}
