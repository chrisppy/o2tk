// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use self::super::super::prelude::{
    DockSize,
    Orientation,
    WidgetTrait,
};

/// The trait the Bar/Dock widgets implement
pub trait DockTrait: DockTraitClone + WidgetTrait {
    /// Retrieve the dock size of this widget
    fn thickness(&self) -> DockSize;
    /// Retrieve the length of this widget
    fn length(&self) -> f32;
    /// Retrieve the orientation of this widget
    fn orientation(&self) -> Orientation;
}

/// Allow the Dock Trait to be cloned
pub trait DockTraitClone {
    /// Clone the boxed widget
    fn clone_box(&self) -> Box<DockTrait>;
}

impl<T> DockTraitClone for T
where
    T: 'static + DockTrait + Clone,
{
    fn clone_box(&self) -> Box<DockTrait> {
        Box::new(self.clone())
    }
}
