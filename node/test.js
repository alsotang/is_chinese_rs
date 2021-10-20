const a = require("./index");
const is_chinese = require("is-chinese");
const string = "我等你的他的".repeat(10);
const chars1000 = "扁担宽，板凳长，扁担".repeat(1);
let buffer = Buffer.from(chars1000);
console.time("label");
for (let i = 0; i < 1000; i++) {
  a.is_chinese_buffer(buffer);
}
console.timeEnd("label");
console.time("label1");
for (let i = 0; i < 1000; i++) {
  a.is_chinese(chars1000);
}
console.timeEnd("label1");

console.time("label2");
for (let i = 0; i < 1000; i++) {
  is_chinese(chars1000);
}
console.timeEnd("label2");
