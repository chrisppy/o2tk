// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::fmt;

/// All the different widget types in the library
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WidgetType {
    /// The Button Widget,
    Button,
    /// The Container Widget
    Container,
    /// The Dock Widget
    Dock,
    /// The Label Widget,
    Label,
    /// The Toolbar Widget
    Toolbar,
    /// Undefined widgets
    Unknown,
    /// The main WindowContainer Widget
    WindowContainer,
}

impl Default for WidgetType {
    fn default() -> Self {
        WidgetType::Unknown
    }
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
