use crate::iracing::string_decoding::cp1252_to_string;
use crate::iracing_basic_solution::header::{Header, VarHeader, VarHeaders};
use crate::iracing_basic_solution::VarData;
use anyhow::{bail, Result};
use std::sync::Arc;
use yaml_rust::{Yaml, YamlLoader};

// TODO: implement debug that's aware of raw data content
#[derive(Clone)]
pub struct SimState {
    _header: Header,
    variables: Arc<VarHeaders>,
    raw_data: Vec<u8>,
    raw_session_info: Vec<u8>,
}

impl SimState {
    pub(super) fn new(
        header: Header,
        variables: Arc<VarHeaders>,
        raw_data: Vec<u8>,
        raw_session_info: Vec<u8>,
    ) -> Self {
        Self {
            _header: header,
            variables,
            raw_data,
            raw_session_info,
        }
    }

    pub fn read<T: VarData>(&self, var: &VarHeader) -> Option<T> {
        self.read_at(0, var)
    }

    pub fn read_at<T: VarData>(&self, idx: usize, var: &VarHeader) -> Option<T> {
        T::parse_from_raw(idx, var, &self.raw_data)
    }

    pub fn read_name<T: VarData>(&self, name: &str) -> Option<T> {
        self.read_name_at(name, 0)
    }

    pub fn read_name_at<T: VarData>(&self, name: &str, idx: usize) -> Option<T> {
        self.read_at(idx, self.variables.get(name)?)
    }

    pub fn variables(&self) -> &VarHeaders {
        &self.variables
    }

    pub fn get_session_info_string(&self) -> Result<String> {
        Ok(cp1252_to_string(&self.raw_session_info)?)
    }

    pub fn get_session_info_yaml(&self) -> Result<Yaml> {
        let data_string = self.get_session_info_string()?;
        let mut items = YamlLoader::load_from_str(&data_string)?;
        if items.is_empty() {
            bail!("Session info did not contain any items");
        }
        Ok(items.swap_remove(0))
    }
}
