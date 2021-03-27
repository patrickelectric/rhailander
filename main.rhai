print("Running: " + system::operating_system());
print("Architecture: " + system::architecture());

companion::run_script("install/network/companion.service");
return false;

helper::run_remote_sh("https://gist.githubusercontent.com/patrickelectric/e53ae492d655bb4b488d4f59da5233f2/raw/0bf95ec390151ca695a8ea8017aa15897f25c653/simple.sh");

print("Going to install companion.");
print("Version: " + companion::version());
print("Remote: " + companion::remote());

if system::architecture() != "arm" {
    throw("Architecture not supported!");
}

if !system::is_root() {
    throw("Program not running as root, aborting!");
}

if !companion::check_remote() {
    throw("Remote is not available: " + companion::remote());
}

const disk_space_in_mb = system::free_disk_space_in_mb();
if  disk_space_in_mb < 4096 {
    throw("Not enough space to install companion, at least 4GB required. Available: " + disk_space_in_mb + "MB");
}

if !helper::run_command("docker --version") {
    print("Docker not available, going to install it.");
    if !helper::run_remote_sh("https://get.docker.com") {
        throw("Failed to install docker, please report this problem.");
    }
}

if !helper::run_command("systemctl enable docker") {
    throw("Failed to enable docker.");
}


const url = "https://raw.githubusercontent.com/patrickelectric/qmlonline/master/README.md";
print("Downloading: " + url);
print("Content: \n"+ network::download(url))
