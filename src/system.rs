use fs2;
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

    pub fn free_disk_space_in_mb() -> i64 {
        return (fs2::available_space("/").unwrap_or_default() / 1024u64.pow(2)) as i64;
    }

    pub fn operating_system() -> String {
        return std::env::consts::OS.into();
    }
}
