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

//! All Widgets, utilities, traits, and enumerations

mod container;
mod dock;
mod label;
mod toolbar;
pub mod window;
mod window_container;

pub use self::{
    container::*,
    dock::*,
    label::*,
    toolbar::*,
    window_container::*,
};
