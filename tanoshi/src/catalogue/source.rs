use std::fmt::Display;

use crate::{context::GlobalContext, local, user};
use async_graphql::{Context, Object, Result, SimpleObject};
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

impl Version {
    pub fn new(v: String) -> Version {
        let split = v.split(".").into_iter().collect::<Vec<&str>>();
        match split.len() {
            0 => Version {
                major: 0,
                minor: 0,
                patch: 0,
            },
            1 => Version {
                major: split[0].parse().unwrap_or(0),
                minor: 0,
                patch: 0,
            },
            2 => Version {
                major: split[0].parse().unwrap_or(0),
                minor: split[1].parse().unwrap_or(0),
                patch: 0,
            },
            _ => Version {
                major: split[0].parse().unwrap_or(0),
                minor: split[1].parse().unwrap_or(0),
                patch: split[2].parse().unwrap_or(0),
            },
        }
    }
}

impl From<String> for Version {
    fn from(v: String) -> Self {
        Version::new(v)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(ord) = self.major.partial_cmp(&other.major) {
            match ord {
                std::cmp::Ordering::Equal => {}
                _ => {
                    return Some(ord);
                }
            }
        }

        if let Some(ord) = self.minor.partial_cmp(&other.minor) {
            match ord {
                std::cmp::Ordering::Equal => {}
                _ => {
                    return Some(ord);
                }
            }
        }

        self.patch.partial_cmp(&other.patch)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourceIndex {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub version: String,
    pub icon: String,
}

impl Into<Source> for SourceIndex {
    fn into(self) -> Source {
        Source {
            id: self.id,
            name: self.name.clone(),
            version: self.version.clone(),
            icon: self.icon,
            need_login: false,
            has_update: false,
        }
    }
}

#[derive(Clone, SimpleObject)]
pub struct Source {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub icon: String,
    pub need_login: bool,
    pub has_update: bool,
}

impl From<tanoshi_lib::data::Source> for Source {
    fn from(s: tanoshi_lib::data::Source) -> Self {
        Self {
            id: s.id,
            name: s.name,
            version: s.version,
            icon: s.icon,
            need_login: s.need_login,
            has_update: false,
        }
    }
}

#[derive(Default)]
pub struct SourceRoot;

#[Object]
impl SourceRoot {
    async fn installed_sources(&self, ctx: &Context<'_>) -> Result<Vec<Source>> {
        let url = "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/index.json"
            .to_string();
        let source_indexes = reqwest::get(url).await?.json::<Vec<SourceIndex>>().await?;

        let extensions = ctx.data::<GlobalContext>()?.extensions.clone();

        let local_source: Source = extensions.detail(local::ID).await?.into();
        let mut sources: Vec<Source> = vec![local_source];

        for index in source_indexes {
            if extensions.exist(index.id).await? {
                let mut source: Source = extensions.detail(index.id).await?.into();
                source.has_update =
                    Version::new(index.version) > Version::new(source.version.clone());
                sources.push(source);
            }
        }
        sources.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(sources)
    }

    async fn available_sources(&self, ctx: &Context<'_>) -> Result<Vec<Source>> {
        let url = "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/index.json"
            .to_string();
        let source_indexes = reqwest::get(url).await?.json::<Vec<SourceIndex>>().await?;
        let extensions = ctx.data::<GlobalContext>()?.extensions.clone();

        let mut sources: Vec<Source> = vec![];
        for index in source_indexes {
            if !extensions.exist(index.id).await? {
                sources.push(index.into());
            }
        }
        Ok(sources)
    }

    async fn source(&self, ctx: &Context<'_>, source_id: i64) -> Result<Source> {
        let exts = ctx.data::<GlobalContext>()?.extensions.clone();
        Ok(exts.detail(source_id).await?.into())
    }
}

#[derive(Default)]
pub struct SourceMutationRoot;

#[Object]
impl SourceMutationRoot {
    async fn install_source(&self, ctx: &Context<'_>, source_id: i64) -> Result<i64> {
        if !user::check_is_admin(ctx) {
            return Err("Forbidden".into());
        }

        let ctx = ctx.data::<GlobalContext>()?;
        let extensions = ctx.extensions.clone();
        if extensions.exist(source_id).await? {
            return Err("source installed, use updateSource to update".into());
        }

        let url = "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/index.json"
            .to_string();
        let source_indexes = reqwest::get(url).await?.json::<Vec<SourceIndex>>().await?;
        let source: SourceIndex = source_indexes
            .iter()
            .find(|index| index.id == source_id)
            .ok_or("source not found")?
            .clone();

        let url = format!(
            "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/{}",
            source.path,
        );

        let raw = reqwest::get(url).await?.bytes().await?;
        extensions.install(source.name, &raw).await?;

        Ok(source.id)
    }

    async fn uninstall_source(&self, ctx: &Context<'_>, source_id: i64) -> Result<i64> {
        if !user::check_is_admin(ctx) {
            return Err("Forbidden".into());
        }

        let ctx = ctx.data::<GlobalContext>()?;
        let extensions = ctx.extensions.clone();

        extensions.unload(source_id).await?;

        Ok(source_id)
    }

    async fn update_source(&self, ctx: &Context<'_>, source_id: i64) -> Result<i64> {
        if !user::check_is_admin(ctx) {
            return Err("Forbidden".into());
        }

        let ctx = ctx.data::<GlobalContext>()?;
        let extensions = ctx.extensions.clone();
        extensions.exist(source_id).await?;

        let url = "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/index.json"
            .to_string();

        let source_indexes = reqwest::get(url).await?.json::<Vec<SourceIndex>>().await?;
        let source: SourceIndex = source_indexes
            .iter()
            .find(|index| index.id == source_id)
            .ok_or("source not found")?
            .clone();

        if extensions.detail(source_id).await?.version == source.version {
            return Err("No new version".into());
        }

        let url = format!(
            "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo/{}",
            source.path,
        );
        let raw = reqwest::get(url).await?.bytes().await?;

        extensions.unload(source_id).await?;
        extensions.install(source.name, &raw).await?;

        Ok(source_id)
    }
}
