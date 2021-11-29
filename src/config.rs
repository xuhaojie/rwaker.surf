use serde::{Deserialize,Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetInfo {
	pub name : String,
	pub mac  : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	#[serde(rename = "url")]
	pub url: String,
	pub user: String,
	pub password: String,
	pub targets: Vec<TargetInfo>,
}

pub fn default() -> Config {
	Config{
		url: String::from("https://user.ddns.net:443"),
		user: String::from("admin"),
		password: String::from("admin"),
		targets: vec![
			TargetInfo{name: String::from("PC"), mac : String::from("11:22:33:44:55:66")},
			TargetInfo{name: String::from("Printer"), mac : String::from("AA:BB:CC:DD:EE:FF")},
		],
	}
}

impl Config {

	pub fn find(&self, name: &String) -> Result<String, String> {
		for target in &self.targets {
			if target.name == *name {
				return Ok(target.mac.to_string());
			}
		}
		Err(format!("target `{}` not find", name))
	}

	pub fn save(&self, file_name: &Path) -> Result<(), String> {
		match std::fs::File::create(file_name) {
			Err(e) => return Err(format!("create file {} failed. {}", file_name.display(), e)),
			Ok(file) => {
				match serde_json::to_writer_pretty(file, &self){
					Ok(()) => Ok(()),
					Err(e) => Err(format!("save config failed. {}", e.to_string()))
				}
			},
		}
	}
}

pub fn load(file_name: &Path) -> Result<Config, String>{
	match std::fs::File::open(file_name) {
		Err(e) => return Err(format!("open {} failed. {}", file_name.display(), e)),
		Ok(file) => {
			match serde_json::from_reader(file){
				Ok(cfg) => Ok(cfg),
				Err(e) => Err(format!("parse config file {} failed. {}", file_name.display(), e)),
			}
		}
	}
}