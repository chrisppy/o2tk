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

/// A structure to hold the color
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color {
    /// The red color value
    red: u16,
    /// The green color value
    green: u16,
    /// The blue color value
    blue: u16,
    /// The alpha value
    alpha: u16,
}

impl Color {
    /// Convert the Color into a hex string
    pub fn into_string(self) -> String {
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

    /// Create the Color from a hex string
    pub fn from_hex<V>(hex: V) -> Result<Self, Error>
    where
        V: Into<String>,
    {
        let hex = hex.into();
        if hex.len() != 9 {
            return Err(err_msg("The hex string must be 9 characters long"));
        } else if !hex.starts_with('#') {
            return Err(err_msg("The hex must start with #"));
        }

        let red = match hex.get(1..3) {
            None => {
                return Err(err_msg("Could not retrieve the Red portion of the hex color"));
            }
            Some(val) => match u16::from_str_radix(val, 16) {
                Err(err) => {
                    return Err(err_msg(format!(
                        "Could not convert the Red portion of the hex color: {}",
                        err
                    )));
                }
                Ok(val) => val,
            },
        };
        let green = match hex.get(3..5) {
            None => {
                return Err(err_msg("Could not retrieve the Green portion of the hex color"));
            }
            Some(val) => match u16::from_str_radix(val, 16) {
                Err(err) => {
                    return Err(err_msg(format!(
                        "Could not convert the Green portion of the hex color: {}",
                        err
                    )));
                }
                Ok(val) => val,
            },
        };
        let blue = match hex.get(5..7) {
            None => {
                return Err(err_msg("Could not retrieve the Blue portion of the hex color"));
            }
            Some(val) => match u16::from_str_radix(val, 16) {
                Err(err) => {
                    return Err(err_msg(format!(
                        "Could not convert the Blue portion of the hex color: {}",
                        err
                    )));
                }
                Ok(val) => val,
            },
        };
        let alpha = match hex.get(7..) {
            None => {
                return Err(err_msg("Could not retrieve the Alpha portion of the hex color"));
            }
            Some(val) => match u16::from_str_radix(val, 16) {
                Err(err) => {
                    return Err(err_msg(format!(
                        "Could not convert the Alpha portion of the hex color: {}",
                        err
                    )));
                }
                Ok(val) => val,
            },
        };

        Ok(Color {
            red,
            green,
            blue,
            alpha,
        })
    }

    /// convert the color as rgba
    pub fn into_rgba(self) -> [f32; 4] {
        let r = f32::from(self.red);
        let g = f32::from(self.green);
        let b = f32::from(self.blue);
        let a = f32::from(self.alpha);
        [r, g, b, a]
    }

    /// convert the color as rgba in a 1.0 scale
    pub fn into_rgba_scale(self) -> [f32; 4] {
        let res = self.into_rgba();
        let r = res[0] / 255.0;
        let g = res[1] / 255.0;
        let b = res[2] / 255.0;
        let a = res[3] / 255.0;
        [r, g, b, a]
    }
}
