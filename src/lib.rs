use reqwest;
use std::{
    error::Error as AnyError,
    fs,
    net::Ipv4Addr,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use utils::{dependencies2, sh2, BoxedErr};

pub struct D5 {
    username: String,
    password: Option<String>,
    pub cache_file: PathBuf,
    //    ip: Result<Ipv4Addr, Box<dyn AnyError>>,
}

impl Default for D5 {
    fn default() -> Self {
        Self {
            username: "dsock".to_string(),
            password: None,
            cache_file: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("/home/dsock/.cache"))
                .join("ip"),
            //          ip: Err(BoxedErr::new("No IP fetched yet")),
        }
    }
}
impl D5 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_user(self, user: &str) -> Self {
        Self {
            username: user.to_string(),
            ..self
        }
    }
    pub fn with_password(self, password: Option<&str>) -> Self {
        Self {
            password: password.map(String::from),
            ..self
        }
    }
    pub fn with_cache_file(self, cache_file: PathBuf) -> Self {
        Self { cache_file, ..self }
    }
    pub fn update_cache(&self, ip: Ipv4Addr) -> Result<(), Box<dyn AnyError>> {
        let cache_dir = dirs::cache_dir().ok_or("Could not open cache directory")?;
        let cache_file = cache_dir.join("ip");
        std::fs::write(&cache_file, ip.to_string())
            .unwrap_or_else(|e| eprintln!("Could not update d5 cache: {}", e));
        Ok(())
    }
    /// Attempts to get the IP address from the cache.
    ///
    /// Returns an error if the cache is not found, cannot be opened, or is older than 1hr.
    pub fn try_ip_from_cache(&self) -> Result<Ipv4Addr, Box<dyn AnyError>> {
        let modified = fs::metadata(&self.cache_file)?.modified()?;

        if SystemTime::now().duration_since(modified)? < Duration::from_secs(1 * 60 * 60) {
            Ok(fs::read_to_string(&self.cache_file)?.parse()?)
        } else {
            Err(BoxedErr::new("Cache too old"))
        }
    }
    /// Fetches an IP from the d5.codesections.com D5 server.  If not provided a
    /// password, it will attempt to prompt the user for one via dmenu.
    ///
    /// Returns an error if it cannot get a password, cannot connect to the
    /// server, or the server returns text than cannot be parsed as an IP (such
    /// as an error message because the password was invalid).
    pub fn try_ip_from_server(self) -> Result<Ipv4Addr, Box<dyn AnyError>> {
        let pw = match self.password.map(String::from) {
            Some(pass) => pass,
            None => {
                dependencies2(vec!["dmenu", "echo"])?;
                let (out, _) = sh2(r##"echo "" | dmenu -p "Password: " -nf "#222222""##)?;
                out.trim().to_string()
            }
        };
        let client = reqwest::blocking::Client::new();
        let url = "https://d5.codesections.com";
        let res = client
            .get(url)
            .basic_auth(self.username, Some(&pw))
            .send()?
            .text()?;
        Ok(res.parse().map_err(|_| BoxedErr::new(&res))?)
    }

    // pub fn try_get_ip(&self) -> Result<Ipv4Addr, Box<dyn AnyError>> {
    //     match Self::try_from_cache(&d5.cache_file) {
    //         Ok(ip) => Ok(ip.inner),
    //         Err(_) => match Self::try_from_server(&d5.username, d5.password) {
    //             Ok(ip) => {
    //                 std::fs::write(&d5.cache_file, ip.to_string())
    //                     .unwrap_or_else(|e| eprintln!("Could not update d5 cache: {}", e));
    //                 Ok(ip.inner)
    //             }
    //             Err(_) => Err(BoxedErr::new("Could not get IP from cache or server")),
    //         },
    //     }
    // }
}

// pub struct D5Ip {
//     pub inner: Ipv4Addr,
// }
// impl std::ops::Deref for D5Ip {
//     type Target = Ipv4Addr;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }
// impl D5Ip {
//     pub fn try_get_ip(d5: D5) -> Result<Ipv4Addr, Box<dyn AnyError>> {
//         match Self::try_from_cache(&d5.cache_file) {
//             Ok(ip) => Ok(ip.inner),
//             Err(_) => match Self::try_from_server(&d5.username, d5.password) {
//                 Ok(ip) => {
//                     std::fs::write(&d5.cache_file, ip.to_string())
//                         .unwrap_or_else(|e| eprintln!("Could not update d5 cache: {}", e));
//                     Ok(ip.inner)
//                 }
//                 Err(_) => Err(BoxedErr::new("Could not get IP from cache or server")),
//             },
//         }
//     }

//     /// Attempts to get the IP address from the cache.
//     ///
//     /// Returns an error if the cache is not found, cannot be opened, or is older than 1hr.
//     pub fn try_from_cache(cache_file: &PathBuf) -> Result<Self, Box<dyn AnyError>> {
//         let modified = fs::metadata(cache_file)?.modified()?;

//         if SystemTime::now().duration_since(modified)? < Duration::from_secs(1 * 60 * 60) {
//             Ok(Self {
//                 inner: fs::read_to_string(cache_file)?.parse()?,
//             })
//         } else {
//             Err(BoxedErr::new("Cache too old"))
//         }
//     }
// }
