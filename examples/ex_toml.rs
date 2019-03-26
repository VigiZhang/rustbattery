extern crate serde;
extern crate toml;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize, Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    // deserialize
    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxx'
        travis = 'yyyyyyyyy'
    "#).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, None);
    assert_eq!(config.keys.github, "xxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyy");

    // serialize
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: Some(8080),
        keys: Keys {
            github: "zzzzzzzzz".to_string(),
            travis: Some("kkkkkkkkk".to_string()),
        },
    };
    let toml = toml::to_string(&config).unwrap();
    println!("{}", toml);
}
