//! Resource providers

use crate::{ctx::Ctx, objs::Sprite};
use std::{convert::Infallible, error::Error};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProviderInfo {
    pub name: String,
    pub desc: Option<String>,
}

pub trait Provider {
    type Error: Error + Send + Sync;

    /// Loads all resources this provider offers into the given `Ctx`, prefixing
    /// IDs with the given namespace..
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error>;
    /// Present any menu needed to load the assets.
    fn present(&mut self) -> Result<(), Self::Error>;
    fn info(&self) -> ProviderInfo;
}

pub struct StaticAssetProvider {
    sprites: &'static [(&'static str, Sprite)],
}

impl StaticAssetProvider {
    pub fn new(sprites: &'static [(&'static str, Sprite)]) -> Self {
        Self { sprites }
    }
}

impl Provider for StaticAssetProvider {
    type Error = Infallible;
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "static asset provider".to_string(),
            desc: Some("loads assets provided statically at compile time".to_string()),
        }
    }
    fn present(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error> {
        for (id, sprite) in self.sprites {
            ctx.add_sprite(format!("{namespace}:{id}"), sprite.clone());
        }
        Ok(())
    }
}
