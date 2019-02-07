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
    Color,
    WidgetTrait,
};

/// The trait the Button widgets implement
pub trait ButtonTrait: ButtonTraitClone + WidgetTrait {
    /// The color when the button is selected
    fn selected_color(&self) -> Color;
}

/// Allow the Button Trait to be cloned
pub trait ButtonTraitClone {
    /// Clone the boxed widget
    fn clone_box(&self) -> Box<ButtonTrait>;
}

impl<T> ButtonTraitClone for T
where
    T: 'static + ButtonTrait + Clone,
{
    fn clone_box(&self) -> Box<ButtonTrait> {
        Box::new(self.clone())
    }
}
