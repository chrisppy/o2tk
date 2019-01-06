// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

#![deny(missing_docs)]

//! TODO

use self::super::{
    err_msg,
    Color,
    Error,
};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::Path,
};
use toml::from_slice;

fn from_reader<R, T>(r: &mut R) -> Result<T, Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;

    match from_slice(&buf) {
        Err(err) => Err(err_msg(format!("Failed to load theme from slice: {}", err))),
        Ok(val) => Ok(val),
    }
}

/// The object containing the api to theme all widgets
#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    bar:              String,
    container:        String,
    dock:             String,
    label_text:       String,
    label_background: String,
    window_container: String,
}

impl Theme {
    /// Use default colors
    pub fn default() -> Self {
        Self {
            bar:              String::from("#161B6DFF"),
            container:        String::from("#161B1DFF"),
            dock:             String::from("#161B6DFF"),
            label_text:       String::from("#FFFFFFFF"),
            label_background: String::from("#161B1DFF"),
            window_container: String::from("#161B3DFF"),
        }
    }

    /// Initialize the theme api
    pub fn build(path: &str) -> Result<Self, Error> {
        let p = Path::new(path);

        match p.extension() {
            None => {
                return Err(err_msg("Error with the file name"));
            }
            Some(v) => match v.to_str() {
                None => {
                    return Err(err_msg("Incorrect file extension, must be toml"));
                }
                Some(v) => {
                    if (v.len() != 4) | !v.contains("toml") {
                        return Err(err_msg("Incorrect file extension, must be toml"));
                    }
                }
            },
        }

        let mut f = match File::open(p) {
            Err(err) => {
                return Err(err_msg(format!("Failed opening theme file: {}", err)));
            }
            Ok(val) => val,
        };

        let theme: Self = match from_reader(&mut f) {
            Err(err) => {
                return Err(err_msg(format!("Failed to load theme: {}", err)));
            }
            Ok(val) => val,
        };

        Ok(theme)
    }

    /// Retrieve the color for the bar color
    pub fn bar_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().bar)
    }

    /// Retrieve the color for the container color
    pub fn container_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().container)
    }

    /// Retrieve the color for the dock color
    pub fn dock_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().dock)
    }

    /// Retrieve the color for the label text
    pub fn label_text_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().label_text)
    }

    /// Retrieve the color for the label background
    pub fn label_background_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().label_background)
    }

    /// Retrieve the color for the window container color
    pub fn window_container_color(&self) -> Result<Color, Error> {
        Color::from_hex(self.clone().window_container)
    }
}
