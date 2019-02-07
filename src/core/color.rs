// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

//! This library is used to convert between different color values (i.e. HEX and RGBA).

use self::super::{
    err_msg,
    Error,
};
use read_color::rgba;
use serde_derive::Deserialize;

/// The Red Green Blue Alpha values of a color.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
pub struct Color {
    /// The red color value
    red: u8,
    /// The green color value
    green: u8,
    /// The blue color value
    blue: u8,
    /// The alpha value
    alpha: u8,
}

impl Color {
    /// Convert the Color into a hex string
    ///
    /// # Examples
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let rust_color = "#B7410EFF";
    /// let c = Color::from_hex(rust_color).unwrap();
    /// assert_eq!(c.into_hex(), String::from(rust_color));
    /// ```
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let rust_color = "#B7410EFF";
    /// let c = Color::from_hex("#B7410E").unwrap();
    /// assert_eq!(c.into_hex(), String::from(rust_color));
    /// ```
    pub fn into_hex(self) -> String {
        let mut r = format!("{:X}", self.red);
        if r.len() == 1 {
            r = format!("0{}", r);
        }

        let mut g = format!("{:X}", self.green);
        if g.len() == 1 {
            g = format!("0{}", g);
        }

        let mut b = format!("{:X}", self.blue);
        if b.len() == 1 {
            b = format!("0{}", b);
        }

        let mut a = format!("{:X}", self.alpha);
        if a.len() == 1 {
            a = format!("0{}", a);
        }

        format!("#{}{}{}{}", r, g, b, a)
    }

    /// Create the Color from a hex string.
    /// Note: if the alpha value of the hex is not passed, the library defaults to FF/255.
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let _rust_color = Color::from_hex("#B7410EFF").unwrap();
    /// ```
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let _rust_color = Color::from_hex("#B7410E").unwrap();
    /// ```
    pub fn from_hex<V>(hex: V) -> Result<Self, Error>
    where
        V: Into<String>,
    {
        let mut hex = hex.into();

        if !hex.starts_with('#') {
            return Err(err_msg("The hex must start with #"));
        }

        let len = hex.len();

        if (len != 9) && (len != 7) {
            return Err(err_msg("The hex string must be either 7 or 9 characters long"));
        }

        if len == 7 {
            hex += "ff";
        }

        hex.remove(0);

        let color = match rgba(&mut hex.chars()) {
            None => {
                return Err(err_msg("Could not convert"));
            }
            Some(val) => val,
        };

        Ok(Color {
            red:   color[0],
            green: color[1],
            blue:  color[2],
            alpha: color[3],
        })
    }

    fn into_rgba_float(self) -> [f32; 4] {
        let r = f32::from(self.red);
        let g = f32::from(self.green);
        let b = f32::from(self.blue);
        let a = f32::from(self.alpha);
        [r, g, b, a]
    }

    /// convert the color as rgba in a 1.0 scale
    ///
    /// # Examples
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let rust_color = "#B7410EFF";
    /// let c = Color::from_hex(rust_color).unwrap();
    /// let rgba_expected = [183.0 / 255.0, 65.0 / 255.0, 14.0 / 255.0, 255.0 / 255.0];
    /// assert_eq!(c.clone().into_scaled_rgba_float(), rgba_expected);
    /// ```
    ///
    /// ```
    /// use o2tk::prelude::Color;
    ///
    /// let rust_color = "#B7410E";
    /// let c = Color::from_hex(rust_color).unwrap();
    /// let rgba_expected = [183.0 / 255.0, 65.0 / 255.0, 14.0 / 255.0, 255.0 / 255.0];
    /// assert_eq!(c.clone().into_scaled_rgba_float(), rgba_expected);
    /// ```
    pub fn into_scaled_rgba_float(self) -> [f32; 4] {
        let res = self.into_rgba_float();
        let r = res[0] / 255.0;
        let g = res[1] / 255.0;
        let b = res[2] / 255.0;
        let a = res[3] / 255.0;
        [r, g, b, a]
    }
}
