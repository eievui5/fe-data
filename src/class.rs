use std::error::Error;
use std::fmt::Write;
use std::path::PathBuf;
use toml::*;

use crate::*;

#[derive(Hash)]
pub struct ClassData {
	// Data
	pub name: String,
	pub desc: String,
	pub texture: PathBuf,
}

impl ClassData {
	pub fn from(name: String, table: Table) -> Result<Self, Box<dyn Error>> {
		let desc = if let Value::String(desc) = &table["desc"] {
			desc.clone()
		} else {
			panic!();
		};
		let texture = if let Value::String(icon) = &table["icon"] {
			PathBuf::from(icon)
		} else {
			panic!();
		};
		Ok(Self {
			name,
			desc,
			texture,
		})
	}

	pub fn with_texture(texture: PathBuf) -> Self {
		Self {
			name: String::new(),
			desc: String::new(),
			texture,
		}
	}

	pub fn to_toml(&self) -> Result<String, Box<dyn Error>> {
		if self.name.len() == 0 {
			Err(FeError::from("A class has a blank name."))?
		}

		let mut toml = String::new();
		write!(toml, "[{:?}]\n", self.name)?;
		write!(toml, "desc = {:?}\n", self.desc)?;
		write!(toml, "icon = {:?}\n", self.texture.display())?;
		Ok(toml)
	}
}
