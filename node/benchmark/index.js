const { Suite } = require("benchmark");
const chalk = require("chalk");

const { is_chinese, is_chinese_buffer } = require("../index");
const is_chinese_js = require("is-chinese");

const chars1000 = "扁担宽，板凳长，扁担".repeat(100);
const chars1000WithS = "扁担宽，板凳长，扁担".repeat(100) + "s";
const allChinese = "扁担宽，板凳长，扁担想绑在板凳上。";
const asciiStart = "ss扁担宽，板凳长，扁担想绑在板凳上。";
const asciiEnd = "扁担宽，板凳长，扁担想绑在板凳上。ss";

const chars1000Buffer = Buffer.from(chars1000);
const chars1000WithSBuffer = Buffer.from(chars1000WithS);
const allChineseBuffer = Buffer.from(allChinese);
const asciiStartBuffer = Buffer.from(asciiStart);
const asciiEndBuffer = Buffer.from(asciiEnd);

const suite = new Suite('isChinese("扁担宽，板凳长，扁担想绑在板凳上。")');

suite
  .add("is_chinese_napi string", () => {
    is_chinese(allChinese);
  })
  .add("is_chinese_napi buffer", () => {
    is_chinese_buffer(allChineseBuffer);
  })
  .add("is_chinese_js", () => {
    is_chinese_js(allChinese);
  })
  .on("cycle", function (event) {
    console.info(String(event.target));
  })
  .on("complete", function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter("fastest").map("name"))}`);
  })
  .run();

// const suite2 = new Suite('isChinese("ss扁担宽，板凳长，扁担想绑在板凳上。")');

// suite2
//   .add("is_chinese_napi string", () => {
//     is_chinese(asciiStart);
//   })
//   .add("is_chinese_napi buffer", () => {
//     is_chinese_buffer(asciiStartBuffer);
//   })
//   .add("is_chinese_js", () => {
//     is_chinese_js(asciiStart);
//   })
//   .on("cycle", function (event) {
//     console.info(String(event.target));
//   })
//   .on("complete", function () {
//     console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter("fastest").map("name"))}`);
//   })
//   .run();

// const suite3 = new Suite('isChinese("扁担宽，板凳长，扁担想绑在板凳上。ss")');

// suite3
//   .add("is_chinese_napi string", () => {
//     is_chinese(asciiEnd);
//   })
//   .add("is_chinese_napi buffer", () => {
//     is_chinese_buffer(asciiEndBuffer);
//   })
//   .add("is_chinese_js", () => {
//     is_chinese_js(asciiEnd);
//   })
//   .on("cycle", function (event) {
//     console.info(String(event.target));
//   })
//   .on("complete", function () {
//     console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter("fastest").map("name"))}`);
//   })
//   .run();

const suite4 = new Suite("isChinese(chars1000) true");

suite4
  .add("is_chinese_napi string", () => {
    is_chinese(chars1000);
  })
  .add("is_chinese_napi buffer", () => {
    is_chinese_buffer(chars1000Buffer);
  })
  .add("is_chinese_js ", () => {
    is_chinese_js(chars1000);
  })
  .on("cycle", function (event) {
    console.info(String(event.target));
  })
  .on("complete", function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter("fastest").map("name"))}`);
  })
  .run();

const suite5 = new Suite("isChinese(chars1000WithS) false");

suite5
  .add("is_chinese_napi string", () => {
    is_chinese(chars1000WithS);
  })
  .add("is_chinese_napi buffer", () => {
    is_chinese_buffer(chars1000WithSBuffer);
  })
  .add("is_chinese_js", () => {
    is_chinese_js(chars1000WithS);
  })
  .on("cycle", function (event) {
    console.info(String(event.target));
  })
  .on("complete", function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter("fastest").map("name"))}`);
  })
  .run();
// .add('isChinese(chars1000) true', function () {
//   isChinese(chars1000);
// })
// .add('isChinese(chars1000WithS) false', function () {
//   isChinese(chars1000WithS);
// })
