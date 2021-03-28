#[macro_use]
extern crate lazy_static;

mod cli;
mod companion;
mod helper;
mod network;
mod script;
mod system;

fn main() -> Result<(), String> {
    cli::init();

    // Run local or global script
    let script_path = std::path::Path::new(cli::script());
    let script_path = match script_path.is_absolute() {
        true => script_path.to_path_buf(),
        false => {
            let base_path = std::env::current_dir().unwrap();
            let path_buf = base_path.join(script_path);
            path_buf.to_path_buf()
        }
    };

    if !script_path.exists() {
        return Err(format!("Failed do not exist: {:?}", &script_path));
    }

    let script_path = script_path.into_os_string().into_string().unwrap();
    if !script::run_script(&script_path) {
        return Err(format!("Operation failed."));
    }

    Ok(())
}
