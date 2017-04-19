#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate porteurbars;
extern crate tempdir;

use tempdir::TempDir;
use clap::{ArgMatches, App, Arg};
use porteurbars::{Result, Template};
use porteurbars::git;

fn run(args: ArgMatches) -> Result<()> {
    let repo = args.value_of("repo").unwrap();
    let url = porteurbars::git::Url::from_str(repo).unwrap();
    let target = args.value_of("target").unwrap_or(".");
    let root = args.value_of("template_root");
    info!("Cloning...");
    let tmp = TempDir::new("porteurbars")?;
    git::clone(url, &tmp, "master")?;
    info!("Applying template...");
    let template = Template::new(&tmp);
    template.apply(target, root)?;
    println!("off you go");
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("portable git hosted project templates")
        .arg(Arg::with_name("repo").value_name("repo")
        .required(true).help("uri of template to apply"))
        .arg(Arg::with_name("target")
            .value_name("target")
            .help("directory to write template output to. defaults to current working directory"))
        .arg(Arg::with_name("template_root")
                .long("template_root")
                .takes_value(true)
                .help("directory within <repo> to use as root. defaults to base of repo"))
        .get_matches();


    if let Err(ref e) = run(args) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "error: {}", e).expect(errmsg);
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
