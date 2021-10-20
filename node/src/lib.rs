#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

#[macro_use]
extern crate napi_derive;

use is_chinese::is_chinese_buffer as is_chinese_buffer_import;
use napi::{CallContext, JsBoolean, JsBuffer, JsNumber, JsObject, JsString, Result};

#[cfg(all(
    target_arch = "x86_64",
    not(target_env = "musl"),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("is_chinese", is_chinese)?;
    exports.create_named_method("is_chinese_buffer", is_chinese_buffer)?;
    Ok(())
}

#[js_function(1)]
fn is_chinese_buffer(ctx: CallContext) -> Result<JsBoolean> {
    let input_data = ctx.get::<JsBuffer>(0)?.into_value()?;
    let result = is_chinese_buffer_import(&input_data);
    ctx.env.get_boolean(result)
}

#[js_function(1)]
fn is_chinese(ctx: CallContext) -> Result<JsBoolean> {
    let input_string = ctx.get::<JsString>(0)?.into_utf8()?;
    let slice = input_string.as_slice();
    let result = is_chinese_buffer_import(slice);
    ctx.env.get_boolean(result)
}
