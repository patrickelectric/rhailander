use rhai::plugin::*;

use crate::cli;
use crate::network::network;

#[export_module]
pub mod companion {
    pub fn check_remote() -> bool {
        return !network::download(&remote()).is_empty();
    }

    pub fn remote() -> String {
        return cli::remote().into();
    }
}
