use std::env;
use std::process;
use tepwgrep::{Config,run};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
