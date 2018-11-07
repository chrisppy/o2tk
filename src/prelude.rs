// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Traits and essential types intended for blanket imports.

pub use self::super::{
    enums::{
        DockSize,
        Orientation,
        Position,
        Run,
        Size,
        WidgetType,
    },
    traits::{
        DockTrait,
        Widget,
    },
    utils::{
        Color,
        Theme,
    },
};
