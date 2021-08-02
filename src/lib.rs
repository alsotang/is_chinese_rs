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
    0x201c, //“
    0x201d, //”
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
];
pub const CHINESE_RANGE: [u32; 24] = [
    // sequence is determine by occurrence probability
    0x4e00, 0x9fff, // CJK Unified Ideographs
    // Chinese punctuation
    0x2018, 0x2019,   //‘
    // 0x2019, 0x2019,   //’
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
    // let a = u32x4::new(100,100, 100, 100);
    // let res = a.ge(u32x4::new(98, 99, 100, 105));
    // println!("{:?}", res.bitmask());
    let is_all_chinese = str.chars().all(|c| {
        CHINESE_RANGE
            .chunks(4)
            .map(u32x4::from_slice_unaligned)
            .any(|slice| {
                let vector = u32x4::splat(c as u32);
                let compare = vector.ge(slice);
                let mask = compare.bitmask();
                mask == 1 || mask == 7

                //     return true
                // } else {
                //     println!("{:?}", mask);
                //     false
                // }
            })
        // .any(|&[start, end]| c as u32 >= start && c as u32 <= end)
    });

    is_all_chinese
}
