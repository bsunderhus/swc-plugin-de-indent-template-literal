mod utils;
mod visitor;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{fn_pass, Pass, Program},
        visit::VisitMutWith,
    },
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
};

pub use utils::{DeIndent, IndentStyle};
pub use visitor::{DeIndentVisitor, DeIndentVisitorConfig};

#[plugin_transform]
fn process_transform(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<DeIndentVisitorConfig>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for de-indent"),
    )
    .expect("invalid config for de-indent");
    program.visit_mut_with(&mut DeIndentVisitor::new(PluginCommentsProxy, config));
    program
}

pub fn de_indent<C>(comments: C, config: DeIndentVisitorConfig) -> impl Pass
where
    C: Comments + Clone + 'static,
{
    fn_pass(move |program| {
        program.visit_mut_with(&mut DeIndentVisitor::new(comments.clone(), config.clone()));
    })
}
