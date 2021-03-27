use nix;
use rhai::plugin::*;

#[export_module]
pub mod system {
    pub fn architecture() -> String {
        return std::env::consts::ARCH.into();
    }

    pub fn is_root() -> bool {
        return nix::unistd::getuid().is_root();
    }

    pub fn operating_system() -> String {
        return std::env::consts::OS.into();
    }
}
