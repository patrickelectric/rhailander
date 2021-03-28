use rhai::plugin::*;

use super::cli;
use super::network::network;

#[export_module]
pub mod companion {
    pub fn check_remote() -> bool {
        return !network::download(&remote()).is_empty();
    }

    //TODO: finish it
    pub fn run_script(script_path: &str) -> bool {
        let mut remote_path = std::path::PathBuf::from(&remote());
        remote_path.push(&version());
        remote_path.push(script_path);
        if let Some(extension) = remote_path.extension() {
            let extension = extension.to_str().unwrap();
            if extension == "rhai" {
                dbg!("RHAI!!!!!!");
            }
            dbg!(extension);
        };
        return true;
    }

    pub fn remote() -> String {
        return cli::remote().into();
    }

    pub fn should_run_docker_purge() -> bool {
        return cli::should_run_docker_purge();
    }

    pub fn version() -> String {
        return cli::version().into();
    }
}
