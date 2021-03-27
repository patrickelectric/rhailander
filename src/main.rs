#[macro_use]
extern crate lazy_static;

use rhai::{Engine, EvalAltResult, plugin::*};

mod cli;
mod companion;
mod network;
mod system;

fn main() -> Result<(), Box<EvalAltResult>>
{
    cli::init();

    let mut engine = Engine::new();
    engine.register_static_module("companion", exported_module!(companion::companion).into());
    engine.register_static_module("network", exported_module!(network::network).into());
    engine.register_static_module("system", exported_module!(system::system).into());

    engine.eval_file::<bool>("main.rhai".into())?;
    Ok(())
}
