// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use self::super::super::{
    err_msg,
    Error,
};
use ron::de::from_reader;
use serde::Deserialize;
use std::{
    fs::File,
    path::Path,
};

/// The object containing the api to theme all widgets
#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    bar:              String,
    container:        String,
    dock:             String,
    window_container: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bar:              String::from("#161B6DFF"),
            container:        String::from("#161B1DFF"),
            dock:             String::from("#161B6DFF"),
            window_container: String::from("#161B3DFF"),
        }
    }
}

impl Theme {
    /// Initialize the theme api
    pub fn build(path: &str) -> Result<Self, Error> {
        let p = Path::new(path);

        match p.extension() {
            None => {
                return Err(err_msg("Error with the file name"));
            }
            Some(v) => match v.to_str() {
                None => {
                    return Err(err_msg("Incorrect file extension, must be o2thm"));
                }
                Some(v) => {
                    if (v.len() != 5) | !v.contains("o2thm") {
                        return Err(err_msg("Incorrect file extension, must be o2thm"));
                    }
                }
            },
        }

        let f = match File::open(p) {
            Err(err) => {
                return Err(err_msg(format!("Failed opening theme file: {}", err)));
            }
            Ok(val) => val,
        };

        let theme: Self = match from_reader(f) {
            Err(err) => {
                return Err(err_msg(format!("Failed to load theme: {}", err)));
            }
            Ok(val) => val,
        };

        Ok(theme)
    }

    /// Retrieve the color for the bar color
    pub fn bar_color(&self) -> String {
        self.clone().bar
    }

    /// Retrieve the color for the container color
    pub fn container_color(&self) -> String {
        self.clone().container
    }

    /// Retrieve the color for the dock color
    pub fn dock_color(&self) -> String {
        self.clone().dock
    }

    /// Retrieve the color for the window container color
    pub fn window_container_color(&self) -> String {
        self.clone().window_container
    }
}
