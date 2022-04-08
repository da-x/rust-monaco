//! Embedded Javascript code for Monaco's language Web Workers.
//! Requires the "workers" feature (enabled by default).
use crate::sys::{Environment, GetWorkerFn};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Worker;

#[cfg(not(target_arch = "wasm32"))]
macro_rules! include_worker {
    ($name: literal) => {
        include_str!(concat!("../../js/", $name))
    };
}

#[cfg(not(target_arch = "wasm32"))]
pub const EDITOR_WORKER: (&str, &str) = ("editor", include_worker!("editor.worker.js"));

#[cfg(not(target_arch = "wasm32"))]
pub const CSS_WORKER: (&str, &str) = ("css", include_worker!("css.worker.js"));

#[cfg(not(target_arch = "wasm32"))]
pub const HTML_WORKER: (&str, &str) = ("html", include_worker!("html.worker.js"));

#[cfg(not(target_arch = "wasm32"))]
pub const JSON_WORKER: (&str, &str) = ("json", include_worker!("json.worker.js"));

#[cfg(not(target_arch = "wasm32"))]
pub fn worker_to_file(inp: (&'static str, &'static str)) -> (String, &'static str) {
    (format!("/static/monaco-editor.{}.0.32.1.js", inp.0), inp.1)
}

fn get_worker(_id: String, label: String) -> Worker {
    let url = format!("/static/monaco-editor.{}.0.32.1.js", label.as_str());
    Worker::new(&url).expect("failed to create worker")
}

fn build_environment() -> Environment {
    let cb = Closure::wrap(Box::new(get_worker) as Box<GetWorkerFn>);
    let env = Environment::default();
    env.set_get_worker(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    env
}

/// Initialize the Monaco environment.
pub fn set_environment() {
    let window = web_sys::window().expect("no global window exists");
    object_set!(window.MonacoEnvironment = build_environment());
}

/// Check if the Monaco environment is set.
pub fn is_environment_set() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(value) = object_get!(try window.MonacoEnvironment) {
            return value.is_truthy();
        }
    }
    false
}

/// Set up the environment if it's not already set up.
pub fn ensure_environment_set() {
    if !is_environment_set() {
        set_environment();
    }
}
