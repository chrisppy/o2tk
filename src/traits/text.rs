// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

/// The trait the Text widgets implement
pub trait TextTrait: TextTraitClone {
    /// Retrieve the lable of this widget
    fn label(&self) -> String;
}

/// Allow the Text Trait to be cloned
pub trait TextTraitClone {
    /// Clone the boxed widget
    fn clone_box(&self) -> Box<TextTrait>;
}

impl<T> TextTraitClone for T
where
    T: 'static + TextTrait + Clone,
{
    fn clone_box(&self) -> Box<TextTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<TextTrait> {
    fn clone(&self) -> Box<TextTrait> {
        self.clone_box()
    }
}
