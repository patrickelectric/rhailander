use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

use rhai::plugin::*;
use shlex;
use tempfile::NamedTempFile;

use crate::network::network;

#[export_module]
pub mod helper {
    pub fn run_command(command: &str) -> bool {
        if command.is_empty() {
            println!("Command is empty.");
            return false;
        }

        let command_and_args = shlex::split(command).unwrap_or_default();
        let (command, args) = command_and_args.split_first().unwrap();

        match std::process::Command::new(command).args(args).status() {
            Ok(exit_status) => return exit_status.success(),
            Err(error) => {
                println!("Failed to run command: {}, reason: {:?}", command, error);
                return false;
            }
        };
    }

    pub fn run_sh_content(content: &str) -> bool {
        // Create temporary file to hold the content
        let mut file = match NamedTempFile::new() {
            Ok(file) => file,
            Err(error) => {
                println!("Error while creating temporary file: {:?}", error);
                return false;
            }
        };

        if let Err(error) = file.write_all(&content.as_bytes()) {
            println!("Error while writing temporary file: {:?}", error);
            return false;
        }

        let (file, path) = match file.keep() {
            Ok(content) => content,
            Err(error) => {
                println!(
                    "Failed to keep temporary file to run script, reason: {:?}",
                    error
                );
                return false;
            }
        };

        // Turn the file executable
        let path = path.to_string_lossy().to_string();
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        if let Err(error) = file.set_permissions(permissions) {
            println!("Failed to set file ({}) as executable: {:?}", &path, error);
            return false;
        }

        // Close file
        drop(file);

        // Execute it
        return run_command(&format!(r#"sh {}"#, &path));
    }

    pub fn run_remote_sh(url: &str) -> bool {
        let content = network::download(url);
        return run_sh_content(&content);
    }
}
