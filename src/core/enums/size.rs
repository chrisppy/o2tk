// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use serde_derive::Deserialize;
use std::fmt;

/// The size of the widget
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Size {
    /// Fill the parent space
    Full,
    /// The size (width, height) percentage of the parent space
    Size(f32, f32),
}

impl Default for Size {
    /// The default Size
    fn default() -> Self {
        Size::Full
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Eq for Size {}

impl Size {
    /// Retrieve the x size
    pub fn x(self) -> f32 {
        match self {
            Size::Full => 100.0,
            Size::Size(x, _) => x,
        }
    }

    /// Retrieve the y size
    pub fn y(self) -> f32 {
        match self {
            Size::Full => 100.0,
            Size::Size(_, y) => y,
        }
    }
}

/// The size of the bar/dock
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum DockSize {
    /// Small Bar/Dock
    Small,
    /// Normal Bar/Dock
    Normal,
    /// Large Bar/Dock
    Large,
    /// XLarge Bar/Dock
    XLarge,
}

impl DockSize {
    /// Convert the DockSize to a float
    pub fn into_f32(self) -> f32 {
        match self {
            DockSize::Small => 3.6,
            DockSize::Normal => 5.4,
            DockSize::Large => 7.2,
            DockSize::XLarge => 9.0,
        }
    }
}

impl Default for DockSize {
    /// The default Size
    fn default() -> Self {
        DockSize::Normal
    }
}

impl fmt::Display for DockSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Eq for DockSize {}
