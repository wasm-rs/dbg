//! # dbg!() for wasm32
//!
//! This is a drop-in replacement for [`std::dbg`], here's how it can be used:
//!
//! ```
//! use wasm_rs_dbg::dbg;
//!
//! fn test() {
//!   dbg!();
//!   dbg!(Some(1));
//! }
//! // ...
//! ```
//!
#[cfg(all(target_arch = "wasm32", feature = "console-debug"))]
pub use web_sys::console::debug_1 as log;
#[cfg(all(target_arch = "wasm32", feature = "console-error"))]
pub use web_sys::console::error_1 as log;
#[cfg(all(target_arch = "wasm32", feature = "console-info"))]
pub use web_sys::console::info_1 as log;
#[cfg(all(target_arch = "wasm32", feature = "console-log"))]
pub use web_sys::console::log_1 as log;
#[cfg(all(target_arch = "wasm32", feature = "console-trace"))]
pub use web_sys::console::trace_1 as log;
#[cfg(all(target_arch = "wasm32", feature = "console-warn"))]
pub use web_sys::console::warn_1 as log;

/// This `dbg!` macro is a drop-in replacement for [`std::dbg`]
///
/// It will use [`web_sys::console`] on `wasm32` architecture. On all
/// other architectures it will call [`std::dbg`].
///
/// It will log at `debug` level by default, and this can configured
/// with this crate's feature flags. For example, specifying `features = ["console-log"]`
/// will send output at `log` level instead. Supported flags are:
///
/// * `console-debug` (default)
/// * `console-log`
/// * `console-error`
/// * `console-info`
/// * `console-trace`
/// * `console-warn`
///
/// ## Note
///
/// These flags are **exclusive**, so it's important to set `default-features` to `false`
#[macro_export]
macro_rules! dbg {
    () => {{
        #[cfg(not(target_arch = "wasm32"))]
        std::dbg!();
        #[cfg(target_arch = "wasm32")]
        $crate::log(&format!("[{}:{}]", std::file!(), std::line!()).into());
    }};
    ($val: expr $(,)?) => {{
        #[cfg(not(target_arch = "wasm32"))]
        std::dbg!($val);
        #[cfg(target_arch = "wasm32")]
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::log(&format!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp).into());
                tmp
            }
        }
    }};
    ($($val: expr),+ $(,)?) => {{
        ($($crate::dbg!($val)),+,)
    }}
}
