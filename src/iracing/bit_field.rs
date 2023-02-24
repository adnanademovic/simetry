use crate::iracing::{VarData, VarType};

pub struct BitField(pub u32);

impl VarData for BitField {
    fn parse(var_type: VarType, data: &[u8]) -> Option<Self> {
        u32::parse(var_type, data).map(Self)
    }
}
