use crate::iracing::{Header, Value, VarData, VarHeader, VarHeaders, VarType};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use yaml_rust::Yaml;

#[derive(Clone)]
pub struct SimState {
    header: Arc<Header>,
    variables: Arc<VarHeaders>,
    raw_data: Vec<u8>,
    session_info: Arc<Yaml>,
}

impl Debug for SimState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimState")
            .field("header", &self.header)
            .field("session_info", &self.session_info)
            .field("data", &DataDebugPrinter(self))
            .finish()
    }
}

impl SimState {
    pub(super) fn new(
        header: Arc<Header>,
        variables: Arc<VarHeaders>,
        raw_data: Vec<u8>,
        session_info: Arc<Yaml>,
    ) -> Self {
        Self {
            header,
            variables,
            raw_data,
            session_info,
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

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn variables(&self) -> &VarHeaders {
        &self.variables
    }

    pub fn session_info(&self) -> &Yaml {
        &self.session_info
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
