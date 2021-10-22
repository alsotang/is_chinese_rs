//! # is_chinese
//!
//! detect whether a string is all Chinese
//!
//! ## example
//!
//! ```rust
//! use is_chinese;
//! assert!(is_chinese::is_chinese("中国"));
//! assert!(!is_chinese::is_chinese("ss"));
//! ```
//!
//!
//!
//!

///
/// ```
/// assert!(is_chinese::is_chinese("中国"));
/// ```
pub fn is_chinese(string: &str) -> bool {
    let has_ascii = is_printable_ascii(string.as_bytes());
    if has_ascii {
        return false;
    }
    string.chars().all(is_chinese_char)
}

// user should guarantee &[u8] is valid utf-8 string
pub fn is_chinese_buffer(bytes: &[u8]) -> bool {
    let has_ascii = is_printable_ascii(bytes);
    if has_ascii {
        return false;
    }
    unsafe {
        std::str::from_utf8_unchecked(bytes)
            .chars()
            .all(is_chinese_char)
    }
}

fn is_printable_ascii(bytes: &[u8]) -> bool {
    (unsafe {
        #[cfg(target_feature = "avx2")]
        {
            is_printable_ascii_avx2(bytes)
        }
        #[cfg(target_feature = "sse2")]
        #[cfg(not(target_feature = "avx2"))]
        {
            is_printable_ascii_sse2(bytes)
        }
        #[cfg(not(target_feature = "sse2"))]
        {
            is_printable_ascii_fallback(bytes)
        }
    }) || has_printable_ascii_in_rest_slice(bytes)
}

fn has_printable_ascii_in_rest_slice(bytes: &[u8]) -> bool {
    use packed_simd::{u8x16, u8x4, u8x8};

    #[cfg(target_feature = "avx2")]
    let lane = 32;
    #[cfg(target_feature = "sse2")]
    #[cfg(not(target_feature = "avx2"))]
    let lane = 16;
    #[cfg(not(target_feature = "sse2"))]
    let lane = 0;

    let reminder = bytes.len() % lane;

    #[cfg(target_feature = "avx2")]
    match reminder {
        16 => u8x16::from_slice_unaligned(&bytes[bytes.len() - 16..])
            .le(u8x16::splat(127))
            .any(),
        8 => u8x8::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x8::splat(127))
            .any(),
        4 => u8x4::from_slice_unaligned(&bytes[bytes.len() - 4..])
            .le(u8x4::splat(127))
            .any(),
        _ => bytes[bytes.len() - reminder..].iter().any(|ch| *ch <= 127),
    }
    #[cfg(target_feature = "sse2")]
    #[cfg(not(target_feature = "avx2"))]
    match reminder {
        8 => u8x8::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x8::splat(127))
            .any(),
        4 => u8x4::from_slice_unaligned(&bytes[bytes.len() - 4..])
            .le(u8x4::splat(127))
            .any(),
        _ => bytes[bytes.len() - reminder..].iter().any(|ch| *ch <= 127),
    }
    #[cfg(not(target_feature = "sse2"))]
    bytes[bytes.len() - reminder..].iter().any(|ch| *ch <= 127)
}

#[target_feature(enable = "sse2")]
unsafe fn is_printable_ascii_sse2(bytes: &[u8]) -> bool {
    use packed_simd::u8x16;
    let mask = u8x16::splat(127);
    bytes
        .chunks_exact(16)
        .map(u8x16::from_slice_unaligned)
        .any(|v| v.le(mask).any())
}

#[target_feature(enable = "avx2")]
unsafe fn is_printable_ascii_avx2(bytes: &[u8]) -> bool {
    use packed_simd::u8x32;
    let mask = u8x32::splat(127);
    bytes
        .chunks_exact(32)
        .map(u8x32::from_slice_unaligned)
        .any(|v| v.le(mask).any())
}

fn is_printable_ascii_fallback(bytes: &[u8]) -> bool {
    bytes.iter().any(|v| *v <= 127)
}
#[inline]
fn is_chinese_char(ch: char) -> bool {
    match ch as u32 {
        0x4e00..=0x9fff => true,
        0xff0c => true,            //，
        0x3002 => true,            //。
        0x3400..=0x4dbf => true,   // CJK Unified Ideographs Extension A
        0x20000..=0x2a6df => true, // CJK Unified Ideographs Extension B
        0x2a700..=0x2b73f => true, // CJK Unified Ideographs Extension C
        0x2b740..=0x2b81f => true, // CJK Unified Ideographs Extension D
        0x2b820..=0x2ceaf => true, // CJK Unified Ideographs Extension E
        0x3300..=0x33ff => true,   // https://en.wikipedia.org/wiki/CJK_Compatibility
        0xfe30..=0xfe4f => true,   // https://en.wikipedia.org/wiki/CJK_Compatibility_Forms
        0xf900..=0xfaff => true,   // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs
        0x2f800..=0x2fa1f => true, // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs_Supplement
        0x00b7 => true,            //·
        0x00d7 => true,            //×
        0x2014 => true,            //—
        0x2018 => true,            //‘
        0x2019 => true,            //’
        0x201c => true,            //“
        0x201d => true,            //”
        0x2026 => true,            //…
        0x3001 => true,            //、
        0x300a => true,            //《
        0x300b => true,            //》
        0x300e => true,            //『
        0x300f => true,            //』
        0x3010 => true,            //【
        0x3011 => true,            //】
        0xff01 => true,            //！
        0xff08 => true,            //（
        0xff09 => true,            //）
        0xff1a => true,            //：
        0xff1f => true,            //？
        _ => false,
    }
}
