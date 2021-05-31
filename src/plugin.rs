use crate::configuration::{Configuration, LanguageVariant};
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{PluginHandler, PluginInfo},
    types::ErrBox,
};
use std::{ffi::OsString, path::Path};

pub struct MyProcessPluginHandler {
    shfmt_path: OsString,
}
impl Default for MyProcessPluginHandler {
    fn default() -> Self {
        let shfmt_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_owned()))
            .map(|p| p.join("shfmt").as_os_str().to_owned())
            .unwrap_or_else(|| OsString::from("shfmt"));

        Self { shfmt_path }
    }
}

impl PluginHandler<Configuration> for MyProcessPluginHandler {
    fn get_plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config_key: "shfmt".into(),
            file_extensions: vec!["sh"].into_iter().map(|x| x.to_string()).collect(),
            help_url: "https://github.com/mvdan/sh/blob/master/cmd/shfmt/shfmt.1.scd#examples".into(),
            config_schema_url: "".into(),
        }
    }

    fn get_license_text(&mut self) -> String {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE")).into()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        crate::configuration::resolve_config(config, global_config)
    }

    fn format_text(
        &mut self,
        file_path: &Path,
        file_text: &str,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String, ErrBox>,
    ) -> Result<String, ErrBox> {
        crate::format_text::format_text(
            &self.shfmt_path,
            file_text,
            config,
            LanguageVariant::from_path(file_path).unwrap_or_default(),
        )
    }
}
