use std::fmt::Display;

pub(crate) const TEN_MBYTES: usize = 1024 * 1024 * 10;

pub(crate) const FOUR_KBYTES: usize = 1024 * 4;

pub(crate) const URL_PARTS_MAX_CHARS: usize = 1024;

pub(crate) const URL_MAX_LENGTH: usize = 4096;

pub(crate) const MAX_REQUEST_LENGTH: usize = if cfg!(debug_assertions) {
    FOUR_KBYTES
} else {
    TEN_MBYTES
};

pub(crate) struct AsciiWhiteSpace;
impl AsciiWhiteSpace {
    pub const fn as_str() -> &'static str {
        " "
    }
    pub const fn len() -> usize {
        1
    }
}

impl Display for AsciiWhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}
