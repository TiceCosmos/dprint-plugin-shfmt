use crate::configuration::{Configuration, LanguageVariant};
use std::{
    ffi::OsStr,
    io::Write,
    process::{Command, Stdio},
};
use tempfile::NamedTempFile;

pub fn format_text(
    shfmt_path: &OsStr,
    file_text: &str,
    config: &Configuration,
    language: LanguageVariant,
) -> Result<String, dprint_core::types::ErrBox> {
    let mut tempfile = NamedTempFile::new()?;
    tempfile.write_all(file_text.as_bytes())?;

    let mut args = vec![];
    config.generate_args(&mut args);
    language.generate_args(&mut args);

    let output = Command::new(shfmt_path)
        .args(&args)
        .arg(tempfile.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use crate::configuration::{Configuration, LanguageVariant};
    use std::path::Path;

    #[test]
    fn format_text() {
        let shfmt_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("shfmt");

        if !shfmt_path.is_file() {
            return;
        }

        let shfmt_path = shfmt_path.as_os_str();
        let language = LanguageVariant::default();

        let config = Configuration {
            indent_width: 2,
            binary_next_line: false,
            switch_case_indent: false,
            space_redirects: false,
            keep_padding: false,
            function_next_line: false,
        };
        vec![
            ("", ""),
            (
                r#"funWithParam(){echo "helloworld"}"#,
                r#"funWithParam() {echo "helloworld"}"#,
            ),
            (
                "ls|while read file;do echo $file;done",
                "ls | while read file; do echo $file; done",
            ),
        ]
        .into_iter()
        .for_each(|(src, dst)| {
            let rst = super::format_text(shfmt_path, src, &config, language).unwrap();
            assert_eq!(rst.trim_end(), dst);
        });
    }
}
