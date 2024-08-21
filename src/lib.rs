pub mod http;

const TEN_MBYTES: usize = 1024 * 1024 * 10;

pub const FOUR_KBYTES: usize = 1024 * 4;

pub const MAX_REQUEST_LENGTH: usize = if cfg!(debug_assertions) {
    FOUR_KBYTES
} else {
    TEN_MBYTES
};
