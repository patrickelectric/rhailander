#!rhailander --script

mprint("## **Starting companion installation process.**");
mprint("Running: **" + system::operating_system() + "**");
mprint("Architecture: **" + system::architecture() + "**");

let companion_version = env("COMPANION_VERSION");
let companion_remote = env("COMPANION_REMOTE");

if companion_version == "" {
    companion_version = "master"
}

if companion_remote == "" {
    companion_remote = "https://raw.githubusercontent.com/bluerobotics/companion-docker"
}


mprint("### Going to install with the following configuration:");
const repository_address = companion_remote + "/" + companion_version + "/";
mprint("- URL: **" + repository_address +  "**" );
mprint(" - Version: **" + companion_version + "**");
mprint(" - Remote: **" + companion_remote + "**" );

// Check if everything is good to go!
if system::architecture() != "arm" {
    throw("Architecture not supported: " + system::architecture());
}

if !system::is_root() {
    throw("Program not running as root, aborting!");
}

if network::download(repository_address + "README.md") == "" {
    throw("Remote is not available: " +repository_address);
}

const disk_space_in_mb = system::free_disk_space_in_mb();
if  disk_space_in_mb < 4096 {
    throw("Not enough space to install companion, at least 4GB required. Available: " + disk_space_in_mb + "MB");
}

// Install and enable docker
if !helper::run_command("docker --version") {
    mprint("**Docker** not available, going to install it.");
    if !helper::run_remote_sh("https://get.docker.com") {
        throw("Failed to install docker, please report this problem.");
    }
}

if !helper::run_command("systemctl enable docker") {
    throw("Failed to enable docker.");
}


const url = "https://raw.githubusercontent.com/patrickelectric/qmlonline/master/README.md";
print("Downloading: " + url);
print("Content: \n"+ network::download(url));


companion::run_script("install/network/companion.service");
return false;

helper::run_remote_sh("https://gist.githubusercontent.com/patrickelectric/e53ae492d655bb4b488d4f59da5233f2/raw/0bf95ec390151ca695a8ea8017aa15897f25c653/simple.sh");