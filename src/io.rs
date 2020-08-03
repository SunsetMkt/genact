//! Module containing functionality for I/O operations.

use std::io::stdout;
use std::io::Write;
use std::time;
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub async fn csleep(length: u64) {
    let sleep_length = time::Duration::from_millis(length as u64);
    async_std::task::sleep(sleep_length).await;
}

#[cfg(target_arch = "wasm32")]
pub async fn csleep(length: u64) {
    let promise = js_sys::Promise::new(&mut move |resolve, _| {
        let window = web_sys::window().expect("should have a Window");
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, length as i32)
            .expect("don't expect error on setTimeout()");
    });

    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[wasm_bindgen(inline_js = "export function write_to_xterm(s) { window.xterm.write(s) }")]
extern "C" {
    pub fn write_to_xterm(s: &str);
}

/// Crossrint `s` with each letter delayed by `delay` milliseconds.
pub async fn dprint<S: Into<String>>(s: S, delay: u64) {
    // Construct a `Vec` of single characters converted to `String`s.
    let string_arr = s
        .into()
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for c in string_arr {
        #[cfg(target_arch = "wasm32")]
        {
            write_to_xterm(&c);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            print!("{}", c);
            stdout().flush().unwrap();
        }

        if delay > 0 {
            csleep(delay).await;
        }
    }
}

/// Print `s`.
pub async fn print<S: Into<String>>(s: S) {
    dprint(s, 0).await;
}

/// Print a newline.
pub async fn newline() {
    print("\r\n").await;
}

/// Return `true` if the given `a` is printable ASCII and `false` if it isn't.
pub fn is_printable_ascii(a: u64) -> bool {
    a >= 0x21 && a <= 0x7e
}

pub async fn cursor_up(n: u64) {
    print(format!("\x1b[{}A", n)).await;
}

pub async fn cursor_left(n: u64) {
    print(format!("\x1b[{}D", n)).await;
}

pub async fn erase_line() {
    print("\x1b[2K\x1b[0G").await;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_terminal_width() -> usize {
    term_size::dimensions().expect("We're not attached to a terminal apparently").0
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "export function get_terminal_width() { return window.xterm.cols }")]
extern "C" {
    pub fn get_terminal_width() -> usize;
}
