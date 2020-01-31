use reqwest;
use std::{
    error::Error,
    fs,
    net::Ipv4Addr,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use utils::{dependencies, sh};

pub struct D5<'a> {
    pub username: &'a str,
    pub password: Option<&'a str>,
    pub cache_file: PathBuf,
}

impl<'a> Default for D5<'a> {
    fn default() -> Self {
        Self {
            username: "dsock",
            password: None,
            cache_file: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("/home/dsock/.cache"))
                .join("ip"),
        }
    }
}
impl<'a> D5<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    /// Gets the d5 IP address, either from the local cache or remote server
    pub fn try_ip(self) -> Result<Ipv4Addr, Box<dyn Error>> {
        match self.try_ip_from_cache() {
            Ok(ip) => Ok(ip),
            Err(_) => Ok(self.try_ip_from_server()?),
        }
    }
    /// Fetches an IP from the d5.codesections.com D5 server.  If not provided a
    /// password, it will attempt to prompt the user for one via dmenu.  After
    /// fetching the IP, records it to the disk cache.
    ///
    /// Returns an error if it cannot get a password, cannot connect to the
    /// server, or the server returns text than cannot be parsed as an IP (such
    /// as an error message because the password was invalid).
    pub fn try_ip_from_server(self) -> Result<Ipv4Addr, Box<dyn Error>> {
        let password = match self.password.as_ref() {
            Some(pass) => pass.to_string(),
            None => {
                dependencies(vec!["dmenu", "echo"])?;
                let (out, _) = sh(r##"echo "" | dmenu -p "Password: " -nf "#222222""##)?;
                out.trim().to_string()
            }
        };
        let client = reqwest::blocking::Client::new();
        let url = "https://d5.codesections.com";
        let res = client
            .get(url)
            .basic_auth(self.username.clone(), Some(&password))
            .send()?
            .text()?;
        let ip = res.parse().map_err(|_| res)?;

        self.update_cache(ip).unwrap_or_else(|e| eprintln!("{}", e));
        Ok(ip)
    }

    fn update_cache(&self, ip: Ipv4Addr) -> Result<(), Box<dyn Error>> {
        let cache_dir = dirs::cache_dir().ok_or("Could not open cache directory")?;
        let cache_file = cache_dir.join(&self.cache_file);
        std::fs::write(&cache_file, ip.to_string())?;
        Ok(())
    }

    fn try_ip_from_cache(&self) -> Result<Ipv4Addr, Box<dyn Error>> {
        let modified = fs::metadata(&self.cache_file)?.modified()?;

        if SystemTime::now().duration_since(modified)? < Duration::from_secs(1 * 60 * 60) {
            Ok(fs::read_to_string(&self.cache_file)?.parse()?)
        } else {
            Err("Cache too old".into())
        }
    }
}
