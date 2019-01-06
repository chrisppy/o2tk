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

/// The orientation of the bar/dock
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
pub enum Orientation {
    /// Position the bar/dock at the top of the screen
    Top,
    /// Position the bar/dock at the left of the screen
    Left,
    /// Position the bar/dock at the right of the screen
    Right,
    /// Position the bar/dock at the bottom of the screen
    Bottom,
}

impl Default for Orientation {
    /// The default Position
    fn default() -> Self {
        Orientation::Top
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
