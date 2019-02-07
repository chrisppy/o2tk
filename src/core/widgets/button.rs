// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::prelude::*;

/// The Container Widget
#[derive(Clone)]
pub struct Button {
    id:             Id,
    parent_id:      Id,
    size:           Size,
    position:       Position,
    color:          Color,
    text_color:     Color,
    text_size:      f32,
    selected_color: Color,
    visible:        bool,
    label:          String,
}

impl WidgetTrait for Button {
    fn widget_type(&self) -> WidgetType {
        WidgetType::Label
    }

    fn id(&self) -> Id {
        self.clone().id
    }

    fn parent_id(&self) -> Option<Id> {
        Some(self.clone().parent_id)
    }

    fn size(&self) -> Size {
        self.size
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn position(&self) -> Position {
        self.position
    }

    fn color(&self) -> Color {
        self.color
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn show(&mut self) {
        self.visible = true;
    }

    fn hide(&mut self) {
        self.visible = false;
    }
}

impl TextTrait for Button {
    fn label(&self) -> String {
        self.clone().label
    }

    fn text_color(&self) -> Color {
        self.text_color
    }

    fn text_size(&self) -> f32 {
        self.text_size
    }
}

impl ButtonTrait for Button {
    fn selected_color(&self) -> Color {
        self.selected_color
    }
}
