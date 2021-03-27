print("Running: " + system::operating_system());
print("Architecture: " + system::architecture());

if system::architecture() != "arm" {
    throw("Architecture not supported!");
}

if !system::is_root() {
    throw("Program not running as root, aborting!");
}

if !companion::check_remote() {
    throw("Remote is not available: " + companion::remote());
}

const url = "https://raw.githubusercontent.com/patrickelectric/qmlonline/master/README.md";
print("Downloading: " + url);
print("Content: \n"+ network::download(url))
