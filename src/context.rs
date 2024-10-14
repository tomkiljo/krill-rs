use handlebars::{Context as DataContext, Handlebars, Helper, HelperResult, Output, RenderContext};
use std::{collections::BTreeMap, env};

use crate::KeyValue;

pub struct Context<'ctx> {
    data: BTreeMap<String, String>,
    registry: Handlebars<'ctx>,
}

impl<'ctx> Context<'ctx> {
    pub fn _new() -> Self {
        let data = BTreeMap::new();
        let registry = Self::new_registry();
        Context { data, registry }
    }

    pub fn from_args(params: Vec<KeyValue>) -> Self {
        let data: BTreeMap<String, String> = params
            .into_iter()
            .map(|KeyValue(key, value)| (key, value))
            .collect();
        let registry = Self::new_registry();
        Context { data, registry }
    }

    fn new_registry() -> Handlebars<'ctx> {
        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);
        registry.set_strict_mode(true);
        registry.register_helper("env", Box::new(env_helper));
        registry
    }

    pub fn render(&self, template: &str) -> anyhow::Result<String> {
        self.registry
            .render_template(template, &self.data)
            .map_err(|e| e.into())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn variable(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

/// Helper function to render environment variables
fn env_helper(
    h: &Helper,
    _: &Handlebars,
    _: &DataContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    let key = param.relative_path().unwrap();
    let value = env::var(key).unwrap_or_default();
    out.write(&value)?;
    Ok(())
}
