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
        .arg(Arg::from("-u --username [USERNAME] 'Username to use with d5'").default_value("dsock"))
        .arg("-p --pass [PASSWORD] 'Password to use with d5'")
        .arg("-f --force 'Ignore the local cache and update the IP address from d5'")
        .get_matches();
    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    let username = cli.value_of("username").expect("default");
    let password = cli.value_of("pass");
    let d5 = D5::new().with_user(username).with_password(password);

    let ip = if cli.is_present("force") {
        d5.try_ip_from_server()?
    } else {
        match d5.try_ip_from_cache() {
            Ok(ip) => ip,
            Err(_) => d5.try_ip_from_server()?,
        }
    };

    print!("{}", ip.to_string());
    Ok(())
}
