use clap::{crate_version, App, Arg, ArgMatches};
use d5_cli::D5;
use std::error::Error;
use utils::Die;

fn main() {
    let cli = App::new("d5")
        .version(crate_version!())
        .about(
            r"Fetch an IP address from the d5 server at d5.codesections.com, or from the local cache.
If no password is supplied, get a password via dmenu.",
        )
        .arg(Arg::from("-u --username [USERNAME] 'Username to use with d5'")
             .takes_value(true))
        .arg(Arg::from("-p --pass [PASSWORD] 'Password to use with d5'")
             .takes_value(true))
        .arg("-f --force 'Ignore the local cache and update the IP address from d5'")
        .arg(Arg::from("--src 'Prints this program's source to stdout'"))
        .get_matches();
    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    if cli.is_present("src") {
        print!(
            "/// main.rs\n{main}\n\n/// lib.rs\n{lib}",
            main = include_str!("main.rs"),
            lib = include_str!("lib.rs")
        );
        return Ok(());
    }
    let mut d5 = D5::new();
    if let Some(username) = cli.value_of("username") {
        d5.username = username;
    }
    if let Some(password) = cli.value_of("pass") {
        d5.password = Some(password)
    }
    let ip = if cli.is_present("force") {
        d5.try_ip_from_server()?
    } else {
        d5.try_ip()?
    };

    print!("{}", ip.to_string());
    Ok(())
}
