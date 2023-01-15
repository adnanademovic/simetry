use crate::windows_util::cp1252_to_string;
use anyhow::{bail, Context, Result};
use yaml_rust::{Yaml, YamlLoader};

pub fn parse_session_info(raw: &[u8]) -> Result<Yaml> {
    let data_string = cp1252_to_string(raw).context("CP1252 decode of session info failed")?;
    let mut items = YamlLoader::load_from_str(&data_string)?;
    if items.is_empty() {
        bail!("Session info did not contain any items");
    }
    Ok(items.swap_remove(0))
}
