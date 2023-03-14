use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml_edit::{Item, Table};
use uuid::Uuid;

pub struct Config {
    pub client_id: Uuid,
    pub hosts: Hosts,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Hosts {
    #[serde(flatten)]
    pub hosts: HashMap<String, Host>,
}

impl Hosts {
    pub fn get<S: AsRef<str>>(&self, hostname: S) -> Option<&Host> {
        self.hosts.get(hostname.as_ref())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Host {
    pub token: String,
    pub user: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub default: bool,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

impl Default for Config {
    fn default() -> Self {
        let mut dir = dirs::home_dir().unwrap();
        dir.push(".config");
        dir.push("oxide");
        let hosts_path = dir.join("hosts.toml");

        let hosts = if let Ok(hosts_content) = read_to_string(hosts_path) {
            toml::from_str(&hosts_content).unwrap()
        } else {
            Default::default()
        };

        Self {
            client_id: Default::default(),
            hosts,
        }
    }
}

impl Config {
    pub fn update_host(&self, hostname: String, host_entry: Host) -> Result<()> {
        let mut dir = dirs::home_dir().unwrap();
        dir.push(".config");
        dir.push("oxide");
        let hosts_path = dir.join("hosts.toml");

        let mut hosts = if let Ok(hosts_content) = read_to_string(&hosts_path) {
            hosts_content.parse::<toml_edit::Document>()?
        } else {
            Default::default()
        };

        let table = hosts
            .entry(&hostname)
            .or_insert_with(|| Item::Table(Table::default()))
            .as_table_mut()
            .unwrap(); // TODO

        let Host {
            token,
            user,
            default,
        } = host_entry;

        table.insert("token", toml_edit::value(token));
        table.insert("user", toml_edit::value(user));

        if default || table.contains_key("default") {
            table.insert("default", toml_edit::value(default));
        }

        std::fs::write(hosts_path, hosts.to_string())?;

        Ok(())
    }
}
