# is_chinese

[![crates.io](https://img.shields.io/crates/v/is_chinese.svg)](https://crates.io/crates/is_chinese)

detect whether a string is all Chinese

## example

```rust
use is_chinese;
assert!(is_chinese::is_chinese("中国"));
assert!(!is_chinese::is_chinese("ss"));
```

## license

MIT