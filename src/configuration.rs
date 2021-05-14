use dprint_core::{
    configuration::{get_unknown_property_diagnostics, get_value},
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
};
use serde::Serialize;
use std::path::Path;
use strum::AsRefStr;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Configuration {
    pub indent_width: u8,
    /// like -bn
    pub binary_next_line: bool,
    /// like -ci
    pub switch_case_indent: bool,
    /// like -sr
    pub space_redirects: bool,
    /// like -kp
    pub keep_padding: bool,
    /// like -fn
    pub function_next_line: bool,
}
impl Configuration {
    pub fn generate_args(&self, buffer: &mut Vec<String>) {
        if self.indent_width != 0 {
            buffer.push("-i".into());
            buffer.push(format!("{}", self.indent_width));
        }

        for (v, s) in &[
            (self.binary_next_line, "-bn"),
            (self.switch_case_indent, "-ci"),
            (self.space_redirects, "-sr"),
            (self.keep_padding, "-kp"),
            (self.function_next_line, "-fn"),
        ] {
            if *v {
                buffer.push(s.to_string());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum LanguageVariant {
    #[strum(serialize = "posix")]
    Posix,
    #[strum(serialize = "bash")]
    Bash,
    #[strum(serialize = "mksh")]
    Mksh,
    #[strum(serialize = "bats")]
    Bats,
}
impl Default for LanguageVariant {
    fn default() -> Self {
        Self::Bash
    }
}
impl LanguageVariant {
    pub fn from_path(file_path: &Path) -> Option<Self> {
        if file_path.extension().and_then(|ext| ext.to_str()) == Some("bat") {
            return Some(Self::Bats);
        }

        let file_content = std::fs::read_to_string(file_path).ok()?;

        let shebang_vec = file_content
            .lines()
            .next()?
            .strip_prefix("#!")?
            .trim()
            .split(' ')
            .collect::<Vec<_>>();

        let shebang_bin = shebang_vec.first()?;

        let shell = if shebang_bin.ends_with("/env") {
            shebang_vec.get(1).copied()
        } else {
            shebang_bin.rsplit_once('/').map(|s| s.1)
        };

        match shell {
            Some("sh") => Some(Self::Posix),
            Some("bash") => Some(Self::Bash),
            Some("mksh") => Some(Self::Mksh),
            _ => None,
        }
    }
    pub fn generate_args(&self, buffer: &mut Vec<String>) {
        buffer.push("-ln".to_owned());
        buffer.push(self.as_ref().to_owned());
    }
}

pub fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut config = config;
    let mut diagnostics = Vec::new();

    let resolved_config = Configuration {
        indent_width: get_value(
            &mut config,
            "indent_width",
            global_config
                .use_tabs
                .filter(|use_tabs| !use_tabs)
                .and(global_config.indent_width)
                .unwrap_or_default(),
            &mut diagnostics,
        ),
        binary_next_line: get_value(&mut config, "binary_next_line", false, &mut diagnostics),
        switch_case_indent: get_value(&mut config, "switch_case_indent", false, &mut diagnostics),
        space_redirects: get_value(&mut config, "space_redirects", false, &mut diagnostics),
        keep_padding: get_value(&mut config, "keep_padding", false, &mut diagnostics),
        function_next_line: get_value(&mut config, "function_next_line", false, &mut diagnostics),
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::{Configuration, LanguageVariant};

    #[test]
    fn configuration_format() {
        let mut args = vec![];

        Configuration {
            indent_width: 0,
            binary_next_line: false,
            switch_case_indent: false,
            space_redirects: false,
            keep_padding: false,
            function_next_line: false,
        }
        .generate_args(&mut args);
        assert!(args.is_empty());

        args.clear();
        Configuration {
            indent_width: 0,
            binary_next_line: true,
            switch_case_indent: true,
            space_redirects: true,
            keep_padding: true,
            function_next_line: true,
        }
        .generate_args(&mut args);
        assert_eq!(args, vec!["-bn", "-ci", "-sr", "-kp", "-fn"]);

        args.clear();
        Configuration {
            indent_width: 4,
            binary_next_line: true,
            switch_case_indent: true,
            space_redirects: true,
            keep_padding: true,
            function_next_line: true,
        }
        .generate_args(&mut args);
        assert_eq!(args, vec!["-i", "4", "-bn", "-ci", "-sr", "-kp", "-fn"]);
    }

    #[test]
    fn get_language_variant_from_path() {
        use tempfile::TempDir;

        let tempdir = TempDir::new().unwrap();

        let file_path = tempdir.path().join("run.sh");
        for (tag, bin) in &[
            (LanguageVariant::Posix, "sh"),
            (LanguageVariant::Bash, "bash"),
            (LanguageVariant::Mksh, "mksh"),
        ] {
            std::fs::write(&file_path, format!("#!/usr/bin/{}", bin)).unwrap();
            assert_eq!(Some(*tag), LanguageVariant::from_path(&file_path));
        }

        let file_path = tempdir.path().join("run.bat");
        assert_eq!(
            Some(LanguageVariant::Bats),
            LanguageVariant::from_path(&file_path)
        );
    }
}
