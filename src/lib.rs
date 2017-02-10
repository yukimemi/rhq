extern crate toml;
extern crate shellexpand;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

pub fn run() -> errors::Result<()> {
    let config = config::Config::new()?;
    warn!("{:?}", config);
    Ok(())
}

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            ShellExpand(::shellexpand::LookupError<::std::env::VarError>);
        }
    }
}

/// aa
mod config {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use toml;
    use shellexpand;

    use errors;

    #[derive(Default, Deserialize)]
    struct RawConfig {
        root: Option<String>,
    }

    #[derive(Debug)]
    pub struct Config {
        root: PathBuf,
    }

    impl Config {
        pub fn new() -> errors::Result<Config> {
            let raw_config: RawConfig = {
                let mut content = String::new();
                File::open("config.toml")?.read_to_string(&mut content)?;
                toml::from_str(&content).ok().unwrap_or_default()
            };

            let root = raw_config.root.unwrap_or("~/.rhq".to_owned());
            let root = shellexpand::full(&root)?;
            let root = PathBuf::from(root.into_owned());

            Ok(Config { root: root })
        }
    }
}
