# is_chinese

[![crates.io](https://img.shields.io/crates/v/is_chinese.svg)](https://crates.io/crates/is_chinese)

detect whether a string is all Chinese

## example

```rust
use is_chinese;
assert!(is_chinese::is_chinese("中国"));
assert!(!is_chinese::is_chinese("ss"));
```

## benchmark
test on MacBook Pro (16-inch, 2019) 2.6 GHz 六核Intel Core i7
```bash
is_chinese("扁担宽，板凳长，扁担想绑在板凳上。")                                         
                        time:   [46.530 ns 49.090 ns 52.108 ns]
                        change: [-30.027% -28.092% -26.047%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

is_chinese("ss扁担宽，板凳长，扁担想绑在板凳上。")                                         
                        time:   [1.8472 ns 1.8525 ns 1.8582 ns]
                        change: [+16.794% +18.154% +19.542%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

is_chinese("扁担宽，板凳长，扁担想绑在板凳上。ss")                                         
                        time:   [9.1091 ns 9.1742 ns 9.2601 ns]
                        change: [+60.777% +63.702% +66.532%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

isChinese(chars1000) true")                                                                             
                        time:   [2.0190 us 2.0553 us 2.1013 us]
                        change: [-31.444% -30.385% -29.334%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

is_chinese("isChinese(chars1001) false")                                                                            
                        time:   [50.272 ns 50.850 ns 51.482 ns]
                        change: [-51.617% -50.891% -50.187%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```

## license

MIT