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
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "console-debug"
))]
pub use web_sys::console::debug_1 as log;
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "console-error"
))]
pub use web_sys::console::error_1 as log;
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "console-info"
))]
pub use web_sys::console::info_1 as log;
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "console-log"))]
pub use web_sys::console::log_1 as log;
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "console-trace"
))]
pub use web_sys::console::trace_1 as log;
#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "console-warn"
))]
pub use web_sys::console::warn_1 as log;

/// This `dbg!` macro is a drop-in replacement for [`std::dbg`]
///
/// It will use [`web_sys::console`] on `wasm32-unknown-unknown`. On all
/// other architectures it will call [`std::dbg`]. It will log at `debug` level by default, and
/// this can configured with this crate's feature flags. For example, specifying `features =
/// ["console-log"]` will send output at `log` level instead. Supported flags are:
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
        #[cfg(any(target_os = "wasi", not(target_arch = "wasm32")))]
        std::dbg!();
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        $crate::log(&format!("[{}:{}]", std::file!(), std::line!()).into());
    }};
    ($val: expr $(,)?) => {{
        #[cfg(any(target_os = "wasi", not(target_arch = "wasm32")))]
        std::dbg!($val);
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
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
