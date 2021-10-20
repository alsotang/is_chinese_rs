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
pub fn is_chinese(str: &str) -> bool {
    // let has_ascii = str.chars().any(|c| c.is_ascii());
    // if has_ascii {
    //     return false;
    // }

    let is_all_chinese = str.chars().all(|c| {
        match c as u32 {
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
    });

    is_all_chinese
}