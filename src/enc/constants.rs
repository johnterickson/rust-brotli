pub const BROTLI_NUM_BLOCK_LEN_SYMBOLS: usize = 26;
pub static kInsBase: [u32; 24] = [
    0, 1, 2, 3, 4, 5, 6, 8, 10, 14, 18, 26, 34, 50, 66, 98, 130, 194, 322, 578, 1090, 2114, 6210,
    22594,
];

pub static kInsExtra: [u32; 24] = [
    0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 12, 14, 24,
];

pub static kCopyBase: [u32; 24] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70, 102, 134, 198, 326, 582, 1094, 2118,
];

pub static kCopyExtra: [u32; 24] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 24,
];

/* Common context lookup table for all context modes. */
static kContextLookup: [u8; 2048] = [
    /* CONTEXT_LSB6, last byte. */
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
    46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 61, 62, 63, /* CONTEXT_LSB6, second last byte, */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* CONTEXT_MSB6, last byte. */
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7,
    8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 13, 13, 14, 14,
    14, 14, 15, 15, 15, 15, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20,
    20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 23, 23, 23, 23, 24, 24, 24, 24, 25, 25, 25, 25, 26, 26,
    26, 26, 27, 27, 27, 27, 28, 28, 28, 28, 29, 29, 29, 29, 30, 30, 30, 30, 31, 31, 31, 31, 32, 32,
    32, 32, 33, 33, 33, 33, 34, 34, 34, 34, 35, 35, 35, 35, 36, 36, 36, 36, 37, 37, 37, 37, 38, 38,
    38, 38, 39, 39, 39, 39, 40, 40, 40, 40, 41, 41, 41, 41, 42, 42, 42, 42, 43, 43, 43, 43, 44, 44,
    44, 44, 45, 45, 45, 45, 46, 46, 46, 46, 47, 47, 47, 47, 48, 48, 48, 48, 49, 49, 49, 49, 50, 50,
    50, 50, 51, 51, 51, 51, 52, 52, 52, 52, 53, 53, 53, 53, 54, 54, 54, 54, 55, 55, 55, 55, 56, 56,
    56, 56, 57, 57, 57, 57, 58, 58, 58, 58, 59, 59, 59, 59, 60, 60, 60, 60, 61, 61, 61, 61, 62, 62,
    62, 62, 63, 63, 63, 63, /* CONTEXT_MSB6, second last byte, */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    /* CONTEXT_UTF8, last byte. */
    /* ASCII range. */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 12, 16, 12, 12, 20, 12, 16, 24, 28, 12, 12, 32, 12, 36, 12, 44, 44, 44, 44, 44, 44, 44, 44,
    44, 44, 32, 32, 24, 40, 28, 12, 12, 48, 52, 52, 52, 48, 52, 52, 52, 48, 52, 52, 52, 52, 52, 48,
    52, 52, 52, 52, 52, 48, 52, 52, 52, 52, 52, 24, 12, 28, 12, 12, 12, 56, 60, 60, 60, 56, 60, 60,
    60, 56, 60, 60, 60, 60, 60, 56, 60, 60, 60, 60, 60, 56, 60, 60, 60, 60, 60, 24, 12, 28, 12, 0,
    /* UTF8 continuation byte range. */
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    /* UTF8 lead byte range. */
    2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
    2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
    /* CONTEXT_UTF8 second last byte. */
    /* ASCII range. */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1,
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 0,
    /* UTF8 continuation byte range. */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /* UTF8 lead byte range. */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    /* CONTEXT_SIGNED, last byte, same as the above values shifted by 3 bits. */
    0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 40, 40, 40, 40,
    40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
    40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 48, 48, 48, 48,
    48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 56, /* CONTEXT_SIGNED, second last byte. */
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7,
];
pub const BROTLI_NUM_HISTOGRAM_DISTANCE_SYMBOLS: usize = 544;
pub const BROTLI_NUM_LITERAL_SYMBOLS: usize = 256;
pub const BROTLI_NUM_COMMAND_SYMBOLS: usize = 704;
pub const BROTLI_WINDOW_GAP: usize = 16;
pub const BROTLI_MAX_NPOSTFIX: usize = 3;
pub const BROTLI_MAX_NDIRECT: usize = 120;
#[inline(always)]
pub fn BROTLI_CONTEXT_LUT(mode: super::histogram::ContextType) -> &'static [u8] {
    &kContextLookup[((mode as usize) << 9)..]
}
pub fn BROTLI_CONTEXT(P1: u8, P2: u8, LUT: &[u8]) -> u8 {
    (LUT)[P1 as usize] | ((LUT)[256 + P2 as usize])
}
pub static kZeroRepsBits: [usize; 704] = [
    0x0, 0x0, 0x0, 0x7, 0x17, 0x27, 0x37, 0x47, 0x57, 0x67, 0x77, 0x770, 0xb87, 0x1387, 0x1b87,
    0x2387, 0x2b87, 0x3387, 0x3b87, 0x397, 0xb97, 0x1397, 0x1b97, 0x2397, 0x2b97, 0x3397, 0x3b97,
    0x3a7, 0xba7, 0x13a7, 0x1ba7, 0x23a7, 0x2ba7, 0x33a7, 0x3ba7, 0x3b7, 0xbb7, 0x13b7, 0x1bb7,
    0x23b7, 0x2bb7, 0x33b7, 0x3bb7, 0x3c7, 0xbc7, 0x13c7, 0x1bc7, 0x23c7, 0x2bc7, 0x33c7, 0x3bc7,
    0x3d7, 0xbd7, 0x13d7, 0x1bd7, 0x23d7, 0x2bd7, 0x33d7, 0x3bd7, 0x3e7, 0xbe7, 0x13e7, 0x1be7,
    0x23e7, 0x2be7, 0x33e7, 0x3be7, 0x3f7, 0xbf7, 0x13f7, 0x1bf7, 0x23f7, 0x2bf7, 0x33f7, 0x3bf7,
    0x1c387, 0x5c387, 0x9c387, 0xdc387, 0x11c387, 0x15c387, 0x19c387, 0x1dc387, 0x1cb87, 0x5cb87,
    0x9cb87, 0xdcb87, 0x11cb87, 0x15cb87, 0x19cb87, 0x1dcb87, 0x1d387, 0x5d387, 0x9d387, 0xdd387,
    0x11d387, 0x15d387, 0x19d387, 0x1dd387, 0x1db87, 0x5db87, 0x9db87, 0xddb87, 0x11db87, 0x15db87,
    0x19db87, 0x1ddb87, 0x1e387, 0x5e387, 0x9e387, 0xde387, 0x11e387, 0x15e387, 0x19e387, 0x1de387,
    0x1eb87, 0x5eb87, 0x9eb87, 0xdeb87, 0x11eb87, 0x15eb87, 0x19eb87, 0x1deb87, 0x1f387, 0x5f387,
    0x9f387, 0xdf387, 0x11f387, 0x15f387, 0x19f387, 0x1df387, 0x1fb87, 0x5fb87, 0x9fb87, 0xdfb87,
    0x11fb87, 0x15fb87, 0x19fb87, 0x1dfb87, 0x1c397, 0x5c397, 0x9c397, 0xdc397, 0x11c397, 0x15c397,
    0x19c397, 0x1dc397, 0x1cb97, 0x5cb97, 0x9cb97, 0xdcb97, 0x11cb97, 0x15cb97, 0x19cb97, 0x1dcb97,
    0x1d397, 0x5d397, 0x9d397, 0xdd397, 0x11d397, 0x15d397, 0x19d397, 0x1dd397, 0x1db97, 0x5db97,
    0x9db97, 0xddb97, 0x11db97, 0x15db97, 0x19db97, 0x1ddb97, 0x1e397, 0x5e397, 0x9e397, 0xde397,
    0x11e397, 0x15e397, 0x19e397, 0x1de397, 0x1eb97, 0x5eb97, 0x9eb97, 0xdeb97, 0x11eb97, 0x15eb97,
    0x19eb97, 0x1deb97, 0x1f397, 0x5f397, 0x9f397, 0xdf397, 0x11f397, 0x15f397, 0x19f397, 0x1df397,
    0x1fb97, 0x5fb97, 0x9fb97, 0xdfb97, 0x11fb97, 0x15fb97, 0x19fb97, 0x1dfb97, 0x1c3a7, 0x5c3a7,
    0x9c3a7, 0xdc3a7, 0x11c3a7, 0x15c3a7, 0x19c3a7, 0x1dc3a7, 0x1cba7, 0x5cba7, 0x9cba7, 0xdcba7,
    0x11cba7, 0x15cba7, 0x19cba7, 0x1dcba7, 0x1d3a7, 0x5d3a7, 0x9d3a7, 0xdd3a7, 0x11d3a7, 0x15d3a7,
    0x19d3a7, 0x1dd3a7, 0x1dba7, 0x5dba7, 0x9dba7, 0xddba7, 0x11dba7, 0x15dba7, 0x19dba7, 0x1ddba7,
    0x1e3a7, 0x5e3a7, 0x9e3a7, 0xde3a7, 0x11e3a7, 0x15e3a7, 0x19e3a7, 0x1de3a7, 0x1eba7, 0x5eba7,
    0x9eba7, 0xdeba7, 0x11eba7, 0x15eba7, 0x19eba7, 0x1deba7, 0x1f3a7, 0x5f3a7, 0x9f3a7, 0xdf3a7,
    0x11f3a7, 0x15f3a7, 0x19f3a7, 0x1df3a7, 0x1fba7, 0x5fba7, 0x9fba7, 0xdfba7, 0x11fba7, 0x15fba7,
    0x19fba7, 0x1dfba7, 0x1c3b7, 0x5c3b7, 0x9c3b7, 0xdc3b7, 0x11c3b7, 0x15c3b7, 0x19c3b7, 0x1dc3b7,
    0x1cbb7, 0x5cbb7, 0x9cbb7, 0xdcbb7, 0x11cbb7, 0x15cbb7, 0x19cbb7, 0x1dcbb7, 0x1d3b7, 0x5d3b7,
    0x9d3b7, 0xdd3b7, 0x11d3b7, 0x15d3b7, 0x19d3b7, 0x1dd3b7, 0x1dbb7, 0x5dbb7, 0x9dbb7, 0xddbb7,
    0x11dbb7, 0x15dbb7, 0x19dbb7, 0x1ddbb7, 0x1e3b7, 0x5e3b7, 0x9e3b7, 0xde3b7, 0x11e3b7, 0x15e3b7,
    0x19e3b7, 0x1de3b7, 0x1ebb7, 0x5ebb7, 0x9ebb7, 0xdebb7, 0x11ebb7, 0x15ebb7, 0x19ebb7, 0x1debb7,
    0x1f3b7, 0x5f3b7, 0x9f3b7, 0xdf3b7, 0x11f3b7, 0x15f3b7, 0x19f3b7, 0x1df3b7, 0x1fbb7, 0x5fbb7,
    0x9fbb7, 0xdfbb7, 0x11fbb7, 0x15fbb7, 0x19fbb7, 0x1dfbb7, 0x1c3c7, 0x5c3c7, 0x9c3c7, 0xdc3c7,
    0x11c3c7, 0x15c3c7, 0x19c3c7, 0x1dc3c7, 0x1cbc7, 0x5cbc7, 0x9cbc7, 0xdcbc7, 0x11cbc7, 0x15cbc7,
    0x19cbc7, 0x1dcbc7, 0x1d3c7, 0x5d3c7, 0x9d3c7, 0xdd3c7, 0x11d3c7, 0x15d3c7, 0x19d3c7, 0x1dd3c7,
    0x1dbc7, 0x5dbc7, 0x9dbc7, 0xddbc7, 0x11dbc7, 0x15dbc7, 0x19dbc7, 0x1ddbc7, 0x1e3c7, 0x5e3c7,
    0x9e3c7, 0xde3c7, 0x11e3c7, 0x15e3c7, 0x19e3c7, 0x1de3c7, 0x1ebc7, 0x5ebc7, 0x9ebc7, 0xdebc7,
    0x11ebc7, 0x15ebc7, 0x19ebc7, 0x1debc7, 0x1f3c7, 0x5f3c7, 0x9f3c7, 0xdf3c7, 0x11f3c7, 0x15f3c7,
    0x19f3c7, 0x1df3c7, 0x1fbc7, 0x5fbc7, 0x9fbc7, 0xdfbc7, 0x11fbc7, 0x15fbc7, 0x19fbc7, 0x1dfbc7,
    0x1c3d7, 0x5c3d7, 0x9c3d7, 0xdc3d7, 0x11c3d7, 0x15c3d7, 0x19c3d7, 0x1dc3d7, 0x1cbd7, 0x5cbd7,
    0x9cbd7, 0xdcbd7, 0x11cbd7, 0x15cbd7, 0x19cbd7, 0x1dcbd7, 0x1d3d7, 0x5d3d7, 0x9d3d7, 0xdd3d7,
    0x11d3d7, 0x15d3d7, 0x19d3d7, 0x1dd3d7, 0x1dbd7, 0x5dbd7, 0x9dbd7, 0xddbd7, 0x11dbd7, 0x15dbd7,
    0x19dbd7, 0x1ddbd7, 0x1e3d7, 0x5e3d7, 0x9e3d7, 0xde3d7, 0x11e3d7, 0x15e3d7, 0x19e3d7, 0x1de3d7,
    0x1ebd7, 0x5ebd7, 0x9ebd7, 0xdebd7, 0x11ebd7, 0x15ebd7, 0x19ebd7, 0x1debd7, 0x1f3d7, 0x5f3d7,
    0x9f3d7, 0xdf3d7, 0x11f3d7, 0x15f3d7, 0x19f3d7, 0x1df3d7, 0x1fbd7, 0x5fbd7, 0x9fbd7, 0xdfbd7,
    0x11fbd7, 0x15fbd7, 0x19fbd7, 0x1dfbd7, 0x1c3e7, 0x5c3e7, 0x9c3e7, 0xdc3e7, 0x11c3e7, 0x15c3e7,
    0x19c3e7, 0x1dc3e7, 0x1cbe7, 0x5cbe7, 0x9cbe7, 0xdcbe7, 0x11cbe7, 0x15cbe7, 0x19cbe7, 0x1dcbe7,
    0x1d3e7, 0x5d3e7, 0x9d3e7, 0xdd3e7, 0x11d3e7, 0x15d3e7, 0x19d3e7, 0x1dd3e7, 0x1dbe7, 0x5dbe7,
    0x9dbe7, 0xddbe7, 0x11dbe7, 0x15dbe7, 0x19dbe7, 0x1ddbe7, 0x1e3e7, 0x5e3e7, 0x9e3e7, 0xde3e7,
    0x11e3e7, 0x15e3e7, 0x19e3e7, 0x1de3e7, 0x1ebe7, 0x5ebe7, 0x9ebe7, 0xdebe7, 0x11ebe7, 0x15ebe7,
    0x19ebe7, 0x1debe7, 0x1f3e7, 0x5f3e7, 0x9f3e7, 0xdf3e7, 0x11f3e7, 0x15f3e7, 0x19f3e7, 0x1df3e7,
    0x1fbe7, 0x5fbe7, 0x9fbe7, 0xdfbe7, 0x11fbe7, 0x15fbe7, 0x19fbe7, 0x1dfbe7, 0x1c3f7, 0x5c3f7,
    0x9c3f7, 0xdc3f7, 0x11c3f7, 0x15c3f7, 0x19c3f7, 0x1dc3f7, 0x1cbf7, 0x5cbf7, 0x9cbf7, 0xdcbf7,
    0x11cbf7, 0x15cbf7, 0x19cbf7, 0x1dcbf7, 0x1d3f7, 0x5d3f7, 0x9d3f7, 0xdd3f7, 0x11d3f7, 0x15d3f7,
    0x19d3f7, 0x1dd3f7, 0x1dbf7, 0x5dbf7, 0x9dbf7, 0xddbf7, 0x11dbf7, 0x15dbf7, 0x19dbf7, 0x1ddbf7,
    0x1e3f7, 0x5e3f7, 0x9e3f7, 0xde3f7, 0x11e3f7, 0x15e3f7, 0x19e3f7, 0x1de3f7, 0x1ebf7, 0x5ebf7,
    0x9ebf7, 0xdebf7, 0x11ebf7, 0x15ebf7, 0x19ebf7, 0x1debf7, 0x1f3f7, 0x5f3f7, 0x9f3f7, 0xdf3f7,
    0x11f3f7, 0x15f3f7, 0x19f3f7, 0x1df3f7, 0x1fbf7, 0x5fbf7, 0x9fbf7, 0xdfbf7, 0x11fbf7, 0x15fbf7,
    0x19fbf7, 0x1dfbf7, 0xe1c387, 0x2e1c387, 0x4e1c387, 0x6e1c387, 0x8e1c387, 0xae1c387, 0xce1c387,
    0xee1c387, 0xe5c387, 0x2e5c387, 0x4e5c387, 0x6e5c387, 0x8e5c387, 0xae5c387, 0xce5c387,
    0xee5c387, 0xe9c387, 0x2e9c387, 0x4e9c387, 0x6e9c387, 0x8e9c387, 0xae9c387, 0xce9c387,
    0xee9c387, 0xedc387, 0x2edc387, 0x4edc387, 0x6edc387, 0x8edc387, 0xaedc387, 0xcedc387,
    0xeedc387, 0xf1c387, 0x2f1c387, 0x4f1c387, 0x6f1c387, 0x8f1c387, 0xaf1c387, 0xcf1c387,
    0xef1c387, 0xf5c387, 0x2f5c387, 0x4f5c387, 0x6f5c387, 0x8f5c387, 0xaf5c387, 0xcf5c387,
    0xef5c387, 0xf9c387, 0x2f9c387, 0x4f9c387, 0x6f9c387, 0x8f9c387, 0xaf9c387, 0xcf9c387,
    0xef9c387, 0xfdc387, 0x2fdc387, 0x4fdc387, 0x6fdc387, 0x8fdc387, 0xafdc387, 0xcfdc387,
    0xefdc387, 0xe1cb87, 0x2e1cb87, 0x4e1cb87, 0x6e1cb87, 0x8e1cb87, 0xae1cb87, 0xce1cb87,
    0xee1cb87, 0xe5cb87, 0x2e5cb87, 0x4e5cb87, 0x6e5cb87, 0x8e5cb87, 0xae5cb87, 0xce5cb87,
    0xee5cb87, 0xe9cb87, 0x2e9cb87, 0x4e9cb87, 0x6e9cb87, 0x8e9cb87, 0xae9cb87, 0xce9cb87,
    0xee9cb87, 0xedcb87, 0x2edcb87, 0x4edcb87, 0x6edcb87, 0x8edcb87, 0xaedcb87, 0xcedcb87,
    0xeedcb87, 0xf1cb87, 0x2f1cb87, 0x4f1cb87, 0x6f1cb87, 0x8f1cb87, 0xaf1cb87, 0xcf1cb87,
    0xef1cb87, 0xf5cb87, 0x2f5cb87, 0x4f5cb87, 0x6f5cb87, 0x8f5cb87, 0xaf5cb87, 0xcf5cb87,
    0xef5cb87, 0xf9cb87, 0x2f9cb87, 0x4f9cb87, 0x6f9cb87, 0x8f9cb87,
];

pub static kZeroRepsDepth: [u32; 704] = [
    0, 4, 8, 7, 7, 7, 7, 7, 7, 7, 7, 11, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28,
];

pub static kUTF8ContextLookup: [u8; 512] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    8, 12, 16, 12, 12, 20, 12, 16, 24, 28, 12, 12, 32, 12, 36, 12, 44, 44, 44, 44, 44, 44, 44, 44,
    44, 44, 32, 32, 24, 40, 28, 12, 12, 48, 52, 52, 52, 48, 52, 52, 52, 48, 52, 52, 52, 52, 52, 48,
    52, 52, 52, 52, 52, 48, 52, 52, 52, 52, 52, 24, 12, 28, 12, 12, 12, 56, 60, 60, 60, 56, 60, 60,
    60, 56, 60, 60, 60, 60, 60, 56, 60, 60, 60, 60, 60, 56, 60, 60, 60, 60, 60, 24, 12, 28, 12, 0,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
    2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1,
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
];

pub static kSigned3BitContextLookup: [u8; 256] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7,
];

pub static kBrotliMinWindowBits: i32 = 10i32;

pub static kBrotliMaxWindowBits: i32 = 24i32;

pub static kCodeLengthDepth: [u8; 18] = [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 0, 4, 4];

pub static kStaticDistanceCodeDepth: [u8; 64] = [
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
];

pub static kCodeLengthBits: [u32; 18] =
    [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 15, 31, 0, 11, 7];
pub static kNonZeroRepsBits: [usize; 704] = [
    0xb, 0x1b, 0x2b, 0x3b, 0x2cb, 0x6cb, 0xacb, 0xecb, 0x2db, 0x6db, 0xadb, 0xedb, 0x2eb, 0x6eb,
    0xaeb, 0xeeb, 0x2fb, 0x6fb, 0xafb, 0xefb, 0xb2cb, 0x1b2cb, 0x2b2cb, 0x3b2cb, 0xb6cb, 0x1b6cb,
    0x2b6cb, 0x3b6cb, 0xbacb, 0x1bacb, 0x2bacb, 0x3bacb, 0xbecb, 0x1becb, 0x2becb, 0x3becb, 0xb2db,
    0x1b2db, 0x2b2db, 0x3b2db, 0xb6db, 0x1b6db, 0x2b6db, 0x3b6db, 0xbadb, 0x1badb, 0x2badb,
    0x3badb, 0xbedb, 0x1bedb, 0x2bedb, 0x3bedb, 0xb2eb, 0x1b2eb, 0x2b2eb, 0x3b2eb, 0xb6eb, 0x1b6eb,
    0x2b6eb, 0x3b6eb, 0xbaeb, 0x1baeb, 0x2baeb, 0x3baeb, 0xbeeb, 0x1beeb, 0x2beeb, 0x3beeb, 0xb2fb,
    0x1b2fb, 0x2b2fb, 0x3b2fb, 0xb6fb, 0x1b6fb, 0x2b6fb, 0x3b6fb, 0xbafb, 0x1bafb, 0x2bafb,
    0x3bafb, 0xbefb, 0x1befb, 0x2befb, 0x3befb, 0x2cb2cb, 0x6cb2cb, 0xacb2cb, 0xecb2cb, 0x2db2cb,
    0x6db2cb, 0xadb2cb, 0xedb2cb, 0x2eb2cb, 0x6eb2cb, 0xaeb2cb, 0xeeb2cb, 0x2fb2cb, 0x6fb2cb,
    0xafb2cb, 0xefb2cb, 0x2cb6cb, 0x6cb6cb, 0xacb6cb, 0xecb6cb, 0x2db6cb, 0x6db6cb, 0xadb6cb,
    0xedb6cb, 0x2eb6cb, 0x6eb6cb, 0xaeb6cb, 0xeeb6cb, 0x2fb6cb, 0x6fb6cb, 0xafb6cb, 0xefb6cb,
    0x2cbacb, 0x6cbacb, 0xacbacb, 0xecbacb, 0x2dbacb, 0x6dbacb, 0xadbacb, 0xedbacb, 0x2ebacb,
    0x6ebacb, 0xaebacb, 0xeebacb, 0x2fbacb, 0x6fbacb, 0xafbacb, 0xefbacb, 0x2cbecb, 0x6cbecb,
    0xacbecb, 0xecbecb, 0x2dbecb, 0x6dbecb, 0xadbecb, 0xedbecb, 0x2ebecb, 0x6ebecb, 0xaebecb,
    0xeebecb, 0x2fbecb, 0x6fbecb, 0xafbecb, 0xefbecb, 0x2cb2db, 0x6cb2db, 0xacb2db, 0xecb2db,
    0x2db2db, 0x6db2db, 0xadb2db, 0xedb2db, 0x2eb2db, 0x6eb2db, 0xaeb2db, 0xeeb2db, 0x2fb2db,
    0x6fb2db, 0xafb2db, 0xefb2db, 0x2cb6db, 0x6cb6db, 0xacb6db, 0xecb6db, 0x2db6db, 0x6db6db,
    0xadb6db, 0xedb6db, 0x2eb6db, 0x6eb6db, 0xaeb6db, 0xeeb6db, 0x2fb6db, 0x6fb6db, 0xafb6db,
    0xefb6db, 0x2cbadb, 0x6cbadb, 0xacbadb, 0xecbadb, 0x2dbadb, 0x6dbadb, 0xadbadb, 0xedbadb,
    0x2ebadb, 0x6ebadb, 0xaebadb, 0xeebadb, 0x2fbadb, 0x6fbadb, 0xafbadb, 0xefbadb, 0x2cbedb,
    0x6cbedb, 0xacbedb, 0xecbedb, 0x2dbedb, 0x6dbedb, 0xadbedb, 0xedbedb, 0x2ebedb, 0x6ebedb,
    0xaebedb, 0xeebedb, 0x2fbedb, 0x6fbedb, 0xafbedb, 0xefbedb, 0x2cb2eb, 0x6cb2eb, 0xacb2eb,
    0xecb2eb, 0x2db2eb, 0x6db2eb, 0xadb2eb, 0xedb2eb, 0x2eb2eb, 0x6eb2eb, 0xaeb2eb, 0xeeb2eb,
    0x2fb2eb, 0x6fb2eb, 0xafb2eb, 0xefb2eb, 0x2cb6eb, 0x6cb6eb, 0xacb6eb, 0xecb6eb, 0x2db6eb,
    0x6db6eb, 0xadb6eb, 0xedb6eb, 0x2eb6eb, 0x6eb6eb, 0xaeb6eb, 0xeeb6eb, 0x2fb6eb, 0x6fb6eb,
    0xafb6eb, 0xefb6eb, 0x2cbaeb, 0x6cbaeb, 0xacbaeb, 0xecbaeb, 0x2dbaeb, 0x6dbaeb, 0xadbaeb,
    0xedbaeb, 0x2ebaeb, 0x6ebaeb, 0xaebaeb, 0xeebaeb, 0x2fbaeb, 0x6fbaeb, 0xafbaeb, 0xefbaeb,
    0x2cbeeb, 0x6cbeeb, 0xacbeeb, 0xecbeeb, 0x2dbeeb, 0x6dbeeb, 0xadbeeb, 0xedbeeb, 0x2ebeeb,
    0x6ebeeb, 0xaebeeb, 0xeebeeb, 0x2fbeeb, 0x6fbeeb, 0xafbeeb, 0xefbeeb, 0x2cb2fb, 0x6cb2fb,
    0xacb2fb, 0xecb2fb, 0x2db2fb, 0x6db2fb, 0xadb2fb, 0xedb2fb, 0x2eb2fb, 0x6eb2fb, 0xaeb2fb,
    0xeeb2fb, 0x2fb2fb, 0x6fb2fb, 0xafb2fb, 0xefb2fb, 0x2cb6fb, 0x6cb6fb, 0xacb6fb, 0xecb6fb,
    0x2db6fb, 0x6db6fb, 0xadb6fb, 0xedb6fb, 0x2eb6fb, 0x6eb6fb, 0xaeb6fb, 0xeeb6fb, 0x2fb6fb,
    0x6fb6fb, 0xafb6fb, 0xefb6fb, 0x2cbafb, 0x6cbafb, 0xacbafb, 0xecbafb, 0x2dbafb, 0x6dbafb,
    0xadbafb, 0xedbafb, 0x2ebafb, 0x6ebafb, 0xaebafb, 0xeebafb, 0x2fbafb, 0x6fbafb, 0xafbafb,
    0xefbafb, 0x2cbefb, 0x6cbefb, 0xacbefb, 0xecbefb, 0x2dbefb, 0x6dbefb, 0xadbefb, 0xedbefb,
    0x2ebefb, 0x6ebefb, 0xaebefb, 0xeebefb, 0x2fbefb, 0x6fbefb, 0xafbefb, 0xefbefb, 0xb2cb2cb,
    0x1b2cb2cb, 0x2b2cb2cb, 0x3b2cb2cb, 0xb6cb2cb, 0x1b6cb2cb, 0x2b6cb2cb, 0x3b6cb2cb, 0xbacb2cb,
    0x1bacb2cb, 0x2bacb2cb, 0x3bacb2cb, 0xbecb2cb, 0x1becb2cb, 0x2becb2cb, 0x3becb2cb, 0xb2db2cb,
    0x1b2db2cb, 0x2b2db2cb, 0x3b2db2cb, 0xb6db2cb, 0x1b6db2cb, 0x2b6db2cb, 0x3b6db2cb, 0xbadb2cb,
    0x1badb2cb, 0x2badb2cb, 0x3badb2cb, 0xbedb2cb, 0x1bedb2cb, 0x2bedb2cb, 0x3bedb2cb, 0xb2eb2cb,
    0x1b2eb2cb, 0x2b2eb2cb, 0x3b2eb2cb, 0xb6eb2cb, 0x1b6eb2cb, 0x2b6eb2cb, 0x3b6eb2cb, 0xbaeb2cb,
    0x1baeb2cb, 0x2baeb2cb, 0x3baeb2cb, 0xbeeb2cb, 0x1beeb2cb, 0x2beeb2cb, 0x3beeb2cb, 0xb2fb2cb,
    0x1b2fb2cb, 0x2b2fb2cb, 0x3b2fb2cb, 0xb6fb2cb, 0x1b6fb2cb, 0x2b6fb2cb, 0x3b6fb2cb, 0xbafb2cb,
    0x1bafb2cb, 0x2bafb2cb, 0x3bafb2cb, 0xbefb2cb, 0x1befb2cb, 0x2befb2cb, 0x3befb2cb, 0xb2cb6cb,
    0x1b2cb6cb, 0x2b2cb6cb, 0x3b2cb6cb, 0xb6cb6cb, 0x1b6cb6cb, 0x2b6cb6cb, 0x3b6cb6cb, 0xbacb6cb,
    0x1bacb6cb, 0x2bacb6cb, 0x3bacb6cb, 0xbecb6cb, 0x1becb6cb, 0x2becb6cb, 0x3becb6cb, 0xb2db6cb,
    0x1b2db6cb, 0x2b2db6cb, 0x3b2db6cb, 0xb6db6cb, 0x1b6db6cb, 0x2b6db6cb, 0x3b6db6cb, 0xbadb6cb,
    0x1badb6cb, 0x2badb6cb, 0x3badb6cb, 0xbedb6cb, 0x1bedb6cb, 0x2bedb6cb, 0x3bedb6cb, 0xb2eb6cb,
    0x1b2eb6cb, 0x2b2eb6cb, 0x3b2eb6cb, 0xb6eb6cb, 0x1b6eb6cb, 0x2b6eb6cb, 0x3b6eb6cb, 0xbaeb6cb,
    0x1baeb6cb, 0x2baeb6cb, 0x3baeb6cb, 0xbeeb6cb, 0x1beeb6cb, 0x2beeb6cb, 0x3beeb6cb, 0xb2fb6cb,
    0x1b2fb6cb, 0x2b2fb6cb, 0x3b2fb6cb, 0xb6fb6cb, 0x1b6fb6cb, 0x2b6fb6cb, 0x3b6fb6cb, 0xbafb6cb,
    0x1bafb6cb, 0x2bafb6cb, 0x3bafb6cb, 0xbefb6cb, 0x1befb6cb, 0x2befb6cb, 0x3befb6cb, 0xb2cbacb,
    0x1b2cbacb, 0x2b2cbacb, 0x3b2cbacb, 0xb6cbacb, 0x1b6cbacb, 0x2b6cbacb, 0x3b6cbacb, 0xbacbacb,
    0x1bacbacb, 0x2bacbacb, 0x3bacbacb, 0xbecbacb, 0x1becbacb, 0x2becbacb, 0x3becbacb, 0xb2dbacb,
    0x1b2dbacb, 0x2b2dbacb, 0x3b2dbacb, 0xb6dbacb, 0x1b6dbacb, 0x2b6dbacb, 0x3b6dbacb, 0xbadbacb,
    0x1badbacb, 0x2badbacb, 0x3badbacb, 0xbedbacb, 0x1bedbacb, 0x2bedbacb, 0x3bedbacb, 0xb2ebacb,
    0x1b2ebacb, 0x2b2ebacb, 0x3b2ebacb, 0xb6ebacb, 0x1b6ebacb, 0x2b6ebacb, 0x3b6ebacb, 0xbaebacb,
    0x1baebacb, 0x2baebacb, 0x3baebacb, 0xbeebacb, 0x1beebacb, 0x2beebacb, 0x3beebacb, 0xb2fbacb,
    0x1b2fbacb, 0x2b2fbacb, 0x3b2fbacb, 0xb6fbacb, 0x1b6fbacb, 0x2b6fbacb, 0x3b6fbacb, 0xbafbacb,
    0x1bafbacb, 0x2bafbacb, 0x3bafbacb, 0xbefbacb, 0x1befbacb, 0x2befbacb, 0x3befbacb, 0xb2cbecb,
    0x1b2cbecb, 0x2b2cbecb, 0x3b2cbecb, 0xb6cbecb, 0x1b6cbecb, 0x2b6cbecb, 0x3b6cbecb, 0xbacbecb,
    0x1bacbecb, 0x2bacbecb, 0x3bacbecb, 0xbecbecb, 0x1becbecb, 0x2becbecb, 0x3becbecb, 0xb2dbecb,
    0x1b2dbecb, 0x2b2dbecb, 0x3b2dbecb, 0xb6dbecb, 0x1b6dbecb, 0x2b6dbecb, 0x3b6dbecb, 0xbadbecb,
    0x1badbecb, 0x2badbecb, 0x3badbecb, 0xbedbecb, 0x1bedbecb, 0x2bedbecb, 0x3bedbecb, 0xb2ebecb,
    0x1b2ebecb, 0x2b2ebecb, 0x3b2ebecb, 0xb6ebecb, 0x1b6ebecb, 0x2b6ebecb, 0x3b6ebecb, 0xbaebecb,
    0x1baebecb, 0x2baebecb, 0x3baebecb, 0xbeebecb, 0x1beebecb, 0x2beebecb, 0x3beebecb, 0xb2fbecb,
    0x1b2fbecb, 0x2b2fbecb, 0x3b2fbecb, 0xb6fbecb, 0x1b6fbecb, 0x2b6fbecb, 0x3b6fbecb, 0xbafbecb,
    0x1bafbecb, 0x2bafbecb, 0x3bafbecb, 0xbefbecb, 0x1befbecb, 0x2befbecb, 0x3befbecb, 0xb2cb2db,
    0x1b2cb2db, 0x2b2cb2db, 0x3b2cb2db, 0xb6cb2db, 0x1b6cb2db, 0x2b6cb2db, 0x3b6cb2db, 0xbacb2db,
    0x1bacb2db, 0x2bacb2db, 0x3bacb2db, 0xbecb2db, 0x1becb2db, 0x2becb2db, 0x3becb2db, 0xb2db2db,
    0x1b2db2db, 0x2b2db2db, 0x3b2db2db, 0xb6db2db, 0x1b6db2db, 0x2b6db2db, 0x3b6db2db, 0xbadb2db,
    0x1badb2db, 0x2badb2db, 0x3badb2db, 0xbedb2db, 0x1bedb2db, 0x2bedb2db, 0x3bedb2db, 0xb2eb2db,
    0x1b2eb2db, 0x2b2eb2db, 0x3b2eb2db, 0xb6eb2db, 0x1b6eb2db, 0x2b6eb2db, 0x3b6eb2db, 0xbaeb2db,
    0x1baeb2db, 0x2baeb2db, 0x3baeb2db, 0xbeeb2db, 0x1beeb2db, 0x2beeb2db, 0x3beeb2db, 0xb2fb2db,
    0x1b2fb2db, 0x2b2fb2db, 0x3b2fb2db, 0xb6fb2db, 0x1b6fb2db, 0x2b6fb2db, 0x3b6fb2db, 0xbafb2db,
    0x1bafb2db, 0x2bafb2db, 0x3bafb2db, 0xbefb2db, 0x1befb2db, 0x2befb2db, 0x3befb2db, 0xb2cb6db,
    0x1b2cb6db, 0x2b2cb6db, 0x3b2cb6db, 0xb6cb6db, 0x1b6cb6db, 0x2b6cb6db, 0x3b6cb6db, 0xbacb6db,
    0x1bacb6db, 0x2bacb6db, 0x3bacb6db, 0xbecb6db, 0x1becb6db, 0x2becb6db, 0x3becb6db, 0xb2db6db,
    0x1b2db6db, 0x2b2db6db, 0x3b2db6db, 0xb6db6db, 0x1b6db6db, 0x2b6db6db, 0x3b6db6db, 0xbadb6db,
    0x1badb6db, 0x2badb6db, 0x3badb6db, 0xbedb6db, 0x1bedb6db, 0x2bedb6db, 0x3bedb6db, 0xb2eb6db,
    0x1b2eb6db, 0x2b2eb6db, 0x3b2eb6db, 0xb6eb6db, 0x1b6eb6db, 0x2b6eb6db, 0x3b6eb6db, 0xbaeb6db,
    0x1baeb6db, 0x2baeb6db, 0x3baeb6db,
];

pub static kNonZeroRepsDepth: [u32; 704] = [
    6, 6, 6, 6, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30,
];

pub static kStaticCommandCodeDepth: [u8; 704] = [
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
];

pub static kStaticDistanceCodeBits: [u16; 64] = [
    0, 32, 16, 48, 8, 40, 24, 56, 4, 36, 20, 52, 12, 44, 28, 60, 2, 34, 18, 50, 10, 42, 26, 58, 6,
    38, 22, 54, 14, 46, 30, 62, 1, 33, 17, 49, 9, 41, 25, 57, 5, 37, 21, 53, 13, 45, 29, 61, 3, 35,
    19, 51, 11, 43, 27, 59, 7, 39, 23, 55, 15, 47, 31, 63,
];
pub static kStaticCommandCodeBits: [u16; 704] = [
    0, 256, 128, 384, 64, 320, 192, 448, 32, 288, 160, 416, 96, 352, 224, 480, 16, 272, 144, 400,
    80, 336, 208, 464, 48, 304, 176, 432, 112, 368, 240, 496, 8, 264, 136, 392, 72, 328, 200, 456,
    40, 296, 168, 424, 104, 360, 232, 488, 24, 280, 152, 408, 88, 344, 216, 472, 56, 312, 184, 440,
    120, 376, 248, 504, 4, 260, 132, 388, 68, 324, 196, 452, 36, 292, 164, 420, 100, 356, 228, 484,
    20, 276, 148, 404, 84, 340, 212, 468, 52, 308, 180, 436, 116, 372, 244, 500, 12, 268, 140, 396,
    76, 332, 204, 460, 44, 300, 172, 428, 108, 364, 236, 492, 28, 284, 156, 412, 92, 348, 220, 476,
    60, 316, 188, 444, 124, 380, 252, 508, 2, 258, 130, 386, 66, 322, 194, 450, 34, 290, 162, 418,
    98, 354, 226, 482, 18, 274, 146, 402, 82, 338, 210, 466, 50, 306, 178, 434, 114, 370, 242, 498,
    10, 266, 138, 394, 74, 330, 202, 458, 42, 298, 170, 426, 106, 362, 234, 490, 26, 282, 154, 410,
    90, 346, 218, 474, 58, 314, 186, 442, 122, 378, 250, 506, 6, 262, 134, 390, 70, 326, 198, 454,
    38, 294, 166, 422, 102, 358, 230, 486, 22, 278, 150, 406, 86, 342, 214, 470, 54, 310, 182, 438,
    118, 374, 246, 502, 14, 270, 142, 398, 78, 334, 206, 462, 46, 302, 174, 430, 110, 366, 238,
    494, 30, 286, 158, 414, 94, 350, 222, 478, 62, 318, 190, 446, 126, 382, 254, 510, 1, 257, 129,
    385, 65, 321, 193, 449, 33, 289, 161, 417, 97, 353, 225, 481, 17, 273, 145, 401, 81, 337, 209,
    465, 49, 305, 177, 433, 113, 369, 241, 497, 9, 265, 137, 393, 73, 329, 201, 457, 41, 297, 169,
    425, 105, 361, 233, 489, 25, 281, 153, 409, 89, 345, 217, 473, 57, 313, 185, 441, 121, 377,
    249, 505, 5, 261, 133, 389, 69, 325, 197, 453, 37, 293, 165, 421, 101, 357, 229, 485, 21, 277,
    149, 405, 85, 341, 213, 469, 53, 309, 181, 437, 117, 373, 245, 501, 13, 269, 141, 397, 77, 333,
    205, 461, 45, 301, 173, 429, 109, 365, 237, 493, 29, 285, 157, 413, 93, 349, 221, 477, 61, 317,
    189, 445, 125, 381, 253, 509, 3, 259, 131, 387, 67, 323, 195, 451, 35, 291, 163, 419, 99, 355,
    227, 483, 19, 275, 147, 403, 83, 339, 211, 467, 51, 307, 179, 435, 115, 371, 243, 499, 11, 267,
    139, 395, 75, 331, 203, 459, 43, 299, 171, 427, 107, 363, 235, 491, 27, 283, 155, 411, 91, 347,
    219, 475, 59, 315, 187, 443, 123, 379, 251, 507, 7, 1031, 519, 1543, 263, 1287, 775, 1799, 135,
    1159, 647, 1671, 391, 1415, 903, 1927, 71, 1095, 583, 1607, 327, 1351, 839, 1863, 199, 1223,
    711, 1735, 455, 1479, 967, 1991, 39, 1063, 551, 1575, 295, 1319, 807, 1831, 167, 1191, 679,
    1703, 423, 1447, 935, 1959, 103, 1127, 615, 1639, 359, 1383, 871, 1895, 231, 1255, 743, 1767,
    487, 1511, 999, 2023, 23, 1047, 535, 1559, 279, 1303, 791, 1815, 151, 1175, 663, 1687, 407,
    1431, 919, 1943, 87, 1111, 599, 1623, 343, 1367, 855, 1879, 215, 1239, 727, 1751, 471, 1495,
    983, 2007, 55, 1079, 567, 1591, 311, 1335, 823, 1847, 183, 1207, 695, 1719, 439, 1463, 951,
    1975, 119, 1143, 631, 1655, 375, 1399, 887, 1911, 247, 1271, 759, 1783, 503, 1527, 1015, 2039,
    15, 1039, 527, 1551, 271, 1295, 783, 1807, 143, 1167, 655, 1679, 399, 1423, 911, 1935, 79,
    1103, 591, 1615, 335, 1359, 847, 1871, 207, 1231, 719, 1743, 463, 1487, 975, 1999, 47, 1071,
    559, 1583, 303, 1327, 815, 1839, 175, 1199, 687, 1711, 431, 1455, 943, 1967, 111, 1135, 623,
    1647, 367, 1391, 879, 1903, 239, 1263, 751, 1775, 495, 1519, 1007, 2031, 31, 1055, 543, 1567,
    287, 1311, 799, 1823, 159, 1183, 671, 1695, 415, 1439, 927, 1951, 95, 1119, 607, 1631, 351,
    1375, 863, 1887, 223, 1247, 735, 1759, 479, 1503, 991, 2015, 63, 1087, 575, 1599, 319, 1343,
    831, 1855, 191, 1215, 703, 1727, 447, 1471, 959, 1983, 127, 1151, 639, 1663, 383, 1407, 895,
    1919, 255, 1279, 767, 1791, 511, 1535, 1023, 2047,
];
