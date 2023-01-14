use crate::iracing::string_decoding::cp1252_to_string;
use crate::iracing_basic_solution::header::{Header, VarHeader, VarHeaders, VarType};
use crate::iracing_basic_solution::{Value, VarData};
use anyhow::{bail, Result};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use yaml_rust::{Yaml, YamlLoader};

// TODO: implement debug that's aware of raw data content
#[derive(Clone)]
pub struct SimState {
    header: Header,
    variables: Arc<VarHeaders>,
    raw_data: Vec<u8>,
    raw_session_info: Vec<u8>,
}

impl Debug for SimState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimState")
            .field("header", &self.header)
            .field(
                "session_info",
                &self
                    .get_session_info_string()
                    .unwrap_or_else(|_| format!("invalid data: {:?}", self.raw_session_info)),
            )
            .field("data", &DataDebugPrinter(self))
            .finish()
    }
}

impl SimState {
    pub(super) fn new(
        header: Header,
        variables: Arc<VarHeaders>,
        raw_data: Vec<u8>,
        raw_session_info: Vec<u8>,
    ) -> Self {
        Self {
            header,
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

struct DataDebugPrinter<'a>(&'a SimState);

impl<'a> Debug for DataDebugPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;
        f.debug_list()
            .entries(self.0.variables().values().map(|header| {
                Variable {
                    name: &header.name,
                    description: &header.desc,
                    datatype: header.var_type,
                    unit: &header.unit,
                    count_as_time: header.count_as_time,
                    data: (0..header.count)
                        .map(|idx| {
                            format!(
                                "{}",
                                self.0
                                    .read_at::<Value>(idx, header)
                                    .unwrap_or(Value::Char(b'?'))
                            )
                        })
                        .join(", "),
                }
            }))
            .finish()
    }
}

#[derive(Debug)]
struct Variable<'a> {
    #[allow(dead_code)]
    name: &'a str,
    #[allow(dead_code)]
    description: &'a str,
    #[allow(dead_code)]
    datatype: VarType,
    #[allow(dead_code)]
    unit: &'a str,
    #[allow(dead_code)]
    count_as_time: bool,
    #[allow(dead_code)]
    data: String,
}
