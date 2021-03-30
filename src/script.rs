use rhai::{plugin::*, Engine, EvalAltResult};
use termimad;

use super::companion;
use super::helper;
use super::network;
use super::system;

pub fn env(name: &str) -> String {
    match std::env::var(name) {
        Ok(value) => value,
        Err(error) => {
            println!(
                "Failed to fetch environment variable ({}), error: {:?}",
                name, error
            );
            return String::new();
        }
    }
}

pub fn markdown_print(string: &str) {
    termimad::print_text(string);
}

pub fn run_script(path: &str) -> bool {
    let mut engine = Engine::new();
    engine.register_static_module("companion", exported_module!(companion::companion).into());
    engine.register_static_module("helper", exported_module!(helper::helper).into());
    engine.register_static_module("network", exported_module!(network::network).into());
    engine.register_static_module("system", exported_module!(system::system).into());

    engine.register_fn("env", env);
    engine.register_fn("run_rhai", run_script);
    engine.register_fn("mprint", markdown_print);

    match engine.eval_file::<()>(std::path::PathBuf::from(path)) {
        Ok(_) => true,
        Err(error) => {
            println!("Failed to run script: {}", path);
            println!("Reason: {:?}", error);
            return false;
        }
    }
}
