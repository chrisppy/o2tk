// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::fmt;

/// The position of the widget
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Position {
    /// Position the widget at the TopLeft of the parent
    TopLeft,
    /// Position the widget at the TopCenter of the parent
    Top,
    /// Position the widget at the TopRight of the parent
    TopRight,
    /// Position the widget at the CenterLeft of the parent
    Left,
    /// Position the widget at the Center of the parent
    Center,
    /// Position the widget at the CenterRight of the parent
    Right,
    /// Position the widget at the BottomLeft of the parent
    BottomLeft,
    /// Position the widget at the BottomCenter of the parent
    Bottom,
    /// Position the widget at the BottomRight of the parent
    BottomRight,
}

impl Default for Position {
    /// The default Position
    fn default() -> Self {
        Position::Center
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
