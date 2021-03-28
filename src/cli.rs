use clap;
use std::sync::Arc;

#[derive(Debug)]
struct Manager<'a> {
    clap_matches: clap::ArgMatches<'a>,
}

lazy_static! {
    static ref MANAGER: Arc<Manager<'static>> = Arc::new(Manager::new());
}

impl Manager<'_> {
    fn new() -> Self {
        Self {
            clap_matches: get_clap_matches(),
        }
    }
}

// Construct our manager, should be done inside main
pub fn init() {
    MANAGER.as_ref();
}

// Check if the verbosity parameter was used
pub fn is_verbose() -> bool {
    return MANAGER.as_ref().clap_matches.is_present("verbose");
}

pub fn should_run_docker_purge() -> bool {
    return !MANAGER.as_ref().clap_matches.is_present("no-docker-purge");
}

pub fn remote() -> &'static str {
    return MANAGER.as_ref().clap_matches.value_of("remote").unwrap();
}

pub fn script() -> &'static str {
    return MANAGER.as_ref().clap_matches.value_of("script").unwrap();
}

pub fn version() -> &'static str {
    return MANAGER.as_ref().clap_matches.value_of("version").unwrap();
}

// Return clap::ArgMatches struct
pub fn matches<'a>() -> clap::ArgMatches<'a> {
    return MANAGER.as_ref().clap_matches.clone();
}

fn get_clap_matches<'a>() -> clap::ArgMatches<'a> {
    let version = format!(
        "{}-{} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_GIT_SHA_SHORT"),
        env!("VERGEN_BUILD_DATE")
    );

    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(version.as_str())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("version")
                .long("version")
                .help("Sets the desired remote version")
                .takes_value(true)
                .default_value("master"),
        )
        .arg(
            clap::Arg::with_name("remote")
                .long("remote")
                .help("Sets the desired remote url to be used")
                .takes_value(true)
                .default_value("https://raw.githubusercontent.com/bluerobotics/companion-docker/"),
        )
        .arg(
            clap::Arg::with_name("no-docker-purge")
                .long("no-docker-purge")
                .help("Do not remove any docker image locally available.")
        )
        .arg(
            clap::Arg::with_name("script")
                .long("script")
                .help("Choose a specific rhai script to run.")
                .takes_value(true)
                .default_value("main.rhai"),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Turn all log categories up to Debug, for more information check RUST_LOG env variable.")
                .takes_value(false),
        );

    return matches.get_matches();
}
