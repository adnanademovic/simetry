const MAX_BUFS: usize = 4;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Header {
    /// this api header version, see IRSDK_VER
    pub ver: i32,
    /// bitfield using irsdk_StatusField
    pub status: i32,
    /// ticks per second (60 or 360 etc)
    pub tick_rate: i32,

    // session information, updated periodically
    /// Incremented when session info changes
    pub session_info_update: i32,
    /// Length in bytes of session info string
    pub session_info_len: i32,
    /// Session info, encoded in YAML format
    pub session_info_offset: i32,

    // State data, output at tick_rate
    /// Length of array pointed to by var_header_offset
    pub num_vars: i32,
    /// Offset to irsdk_varHeader[num_vars] array, Describes the variables received in var_buf
    pub var_header_offset: i32,

    /// Less or equal to IRSDK_MAX_BUFS (3 for now)
    pub num_buf: i32,
    /// Length in bytes for one line
    pub buf_len: i32,
    /// (16 byte align)
    pub pad: [i32; 2],
    /// Buffers of data being written to
    pub var_buf: [VarBuf; MAX_BUFS],
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct VarBuf {
    /// Used to detect changes in data
    pub tick_count: i32,
    /// Offset from header
    pub buf_offset: i32,
    /// (16 byte align)
    pub pad: [i32; 2],
}
