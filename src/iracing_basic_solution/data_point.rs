use super::header::VarHeader;
use super::VarData;
use crate::iracing::Header;
use std::collections::HashMap;

pub struct DataPoint<'a> {
    pub vars: &'a HashMap<String, VarHeader>,
    pub header: &'a Header,
    pub raw_data: &'a [u8],
}

impl<'a> DataPoint<'a> {
    pub fn read<T: VarData>(&self, var: &VarHeader) -> Option<T> {
        self.read_at(0, var)
    }

    pub fn read_at<T: VarData>(&self, idx: usize, var: &VarHeader) -> Option<T> {
        T::parse_from_raw(idx, var, self.raw_data)
    }

    pub fn read_name<T: VarData>(&self, name: &str) -> Option<T> {
        self.read_name_at(name, 0)
    }

    pub fn read_name_at<T: VarData>(&self, name: &str, idx: usize) -> Option<T> {
        self.read_at(idx, self.vars.get(name)?)
    }
}
