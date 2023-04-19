// https://en.wikipedia.org/wiki/Racing_flags

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RacingFlags {
    pub green: bool,
    pub yellow: bool,
    pub blue: bool,
    pub white: bool,
    pub red: bool,
    pub black: bool,
    pub checkered: bool,
    pub meatball: bool,
    pub black_and_white: bool,
    pub start_ready: bool,
    pub start_set: bool,
    pub start_go: bool,
}
