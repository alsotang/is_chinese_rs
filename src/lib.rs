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
const CHINESE_SINGLE_CHAR: [u32; 20] = [
    0xff0c, //，
    0x3002, //。
    0x00b7, //·
    0x00d7, //×
    0x2014, //—
    0x2026, //…
    0x3001, //、
    0x300a, //《
    0x300b, //》
    0x300e, //『
    0x300f, //』
    0x3010, //【
    0x3011, //】
    0xff01, //！
    0xff08, //（
    0xff09, //）
    0xff1a, //：
    0xff1b, //；
    0xff1f, //？
    0xff1f, //HACK: ？ this duplicate for fit simd lane
];
// [2018, 2019, 3300, 33ff, 3400, 4dbf, 4e00, 9fff, f900, faff, fe30, fe4f, 20000, 2a6df, 2a700, 2b73f, 2b740, 2b81f, 2b820, 2ceaf, 2f800, 2fa1f]
pub const CHINESE_RANGE: [u32; 24] = [
    // sequence is determine by occurrence probability
    0x4e00, 0x9fff, // CJK Unified Ideographs
    0x2018, 0x2019, //‘- ’
    0x3400, 0x4dbf, // CJK Unified Ideographs Extension A
    0x20000, 0x2a6df, // CJK Unified Ideographs Extension B
    0x2a700, 0x2b73f, // CJK Unified Ideographs Extension C
    0x2b740, 0x2b81f, // CJK Unified Ideographs Extension D
    0x2b820, 0x2ceaf, // CJK Unified Ideographs Extension E
    0xf900, 0xfaff, // CJK Compatibility Ideographs
    0x3300, 0x33ff, // https://en.wikipedia.org/wiki/CJK_Compatibility
    0xfe30, 0xfe4f, // https://en.wikipedia.org/wiki/CJK_Compatibility_Forms
    0xf900, 0xfaff, // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs
    0x2f800, 0x2fa1f, // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs_Supplement
];

///
/// ```
/// assert!(is_chinese::is_chinese("中国"));
/// ```
pub fn is_chinese(str: &str) -> bool {
    let has_ascii = str.chars().any(|c| c as u32 <= 255);

    if has_ascii {
        return false;
    }

    use packed_simd::u32x4;
    let is_all_chinese = str.chars().all(|c| {
        let char_vec = u32x4::splat(c as u32);
        CHINESE_SINGLE_CHAR
            .chunks(4)
            .map(u32x4::from_slice_unaligned)
            .any(|char_vec| char_vec.eq(char_vec).any())
            || CHINESE_RANGE
                .chunks(4)
                .map(u32x4::from_slice_unaligned)
                .any(|slice| {
                    let mask = char_vec.ge(slice).bitmask();
                    mask == 1 || mask == 7
                })
    });

    is_all_chinese
}
