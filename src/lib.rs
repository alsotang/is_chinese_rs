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
    let has_ascii = is_printable_ascii(string);
    if has_ascii {
        return false;
    }
    string.chars().all(is_chinese_char)
}

fn is_printable_ascii(string: &str) -> bool {
    let bytes = string.as_bytes();
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
    }) || has_printable_ascii_in_rest_slice(bytes, string)
}

fn has_printable_ascii_in_rest_slice(bytes: &[u8], string: &str) -> bool {
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
        16 => u8x16::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x16::splat(127))
            .any(),
        8 => u8x8::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x8::splat(127))
            .any(),
        4 => u8x4::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x4::splat(127))
            .any(),
        _ => bytes[string.len() - reminder..].iter().any(|ch| *ch <= 127),
    }
    #[cfg(target_feature = "sse2")]
    #[cfg(not(target_feature = "avx2"))]
    match reminder {
        8 => u8x8::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x8::splat(127))
            .any(),
        4 => u8x4::from_slice_unaligned(&bytes[bytes.len() - 8..])
            .le(u8x4::splat(127))
            .any(),
        _ => bytes[string.len() - reminder..].iter().any(|ch| *ch <= 127),
    }
    #[cfg(not(target_feature = "sse2"))]
    bytes[string.len() - reminder..].iter().any(|ch| *ch <= 127)
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
        0x4e00..=0x9fff => return true,
        0xff0c => {
            return true;
        }
        0x3002 => {
            return true;
        }
        0x3400..=0x4dbf => return true, // CJK Unified Ideographs Extension A
        0x20000..=0x2a6df => return true, // CJK Unified Ideographs Extension B
        0x2a700..=0x2b73f => return true, // CJK Unified Ideographs Extension C
        0x2b740..=0x2b81f => return true, // CJK Unified Ideographs Extension D
        0x2b820..=0x2ceaf => return true, // CJK Unified Ideographs Extension E
        0x3300..=0x33ff => return true, // https://en.wikipedia.org/wiki/CJK_Compatibility
        0xfe30..=0xfe4f => return true, // https://en.wikipedia.org/wiki/CJK_Compatibility_Forms
        0xf900..=0xfaff => return true, // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs
        0x2f800..=0x2fa1f => return true, // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs_Supplement
        0x00b7 => return true,            //·
        0x00d7 => return true,            //×
        0x2014 => return true,            //—
        0x2018 => return true,            //‘
        0x2019 => return true,            //’
        0x201c => return true,            //“
        0x201d => return true,            //”
        0x2026 => return true,            //…
        0x3001 => return true,            //、
        0x300a => return true,            //《
        0x300b => return true,            //》
        0x300e => return true,            //『
        0x300f => return true,            //』
        0x3010 => return true,            //【
        0x3011 => return true,            //】
        0xff01 => return true,            //！
        0xff08 => return true,            //（
        0xff09 => return true,            //）
        0xff1a => return true,            //：
        0xff1f => return true,            //？
        _ => false,
    }
}
