# BENCHMARK
```bash
is_chinese_napi string x 5,295,950 ops/sec ±1.39% (89 runs sampled)
is_chinese_napi buffer x 6,370,853 ops/sec ±1.26% (90 runs sampled)
is_chinese_js x 10,824,426 ops/sec ±1.73% (85 runs sampled)
isChinese("扁担宽，板凳长，扁担想绑在板凳上。") bench suite: Fastest is is_chinese_js
is_chinese_napi string x 6,539,979 ops/sec ±1.83% (87 runs sampled)
is_chinese_napi buffer x 8,684,546 ops/sec ±1.63% (87 runs sampled)
is_chinese_js x 43,698,976 ops/sec ±1.50% (87 runs sampled)
isChinese("ss扁担宽，板凳长，扁担想绑在板凳上。") bench suite: Fastest is is_chinese_js
is_chinese_napi string x 6,300,983 ops/sec ±1.25% (90 runs sampled)
is_chinese_napi buffer x 8,043,654 ops/sec ±1.34% (85 runs sampled)
is_chinese_js x 30,377,247 ops/sec ±2.38% (88 runs sampled)
isChinese("扁担宽，板凳长，扁担想绑在板凳上。ss") bench suite: Fastest is is_chinese_js
is_chinese_napi string x 180,143 ops/sec ±1.51% (91 runs sampled)
is_chinese_napi buffer x 442,211 ops/sec ±1.40% (92 runs sampled)
is_chinese_js  x 351,385 ops/sec ±1.22% (87 runs sampled)
isChinese(chars1000) true bench suite: Fastest is is_chinese_napi buffer
is_chinese_napi string x 289,675 ops/sec ±1.22% (90 runs sampled)
is_chinese_napi buffer x 5,796,385 ops/sec ±1.43% (89 runs sampled)
is_chinese_js x 1,771,844 ops/sec ±1.15% (93 runs sampled)
isChinese(chars1000WithS) false bench suite: Fastest is is_chinese_napi buffer
```