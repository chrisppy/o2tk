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

/// The Toolbar Widget
#[derive(Clone)]
pub struct Toolbar {
    id:          Id,
    parent_id:   Id,
    size:        Size,
    position:    Position,
    color:       Color,
    visible:     bool,
    thickness:   DockSize,
    length:      f32,
    orientation: Orientation,
}

impl WidgetTrait for Toolbar {
    fn widget_type(&self) -> WidgetType {
        WidgetType::Toolbar
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

impl DockTrait for Toolbar {
    fn thickness(&self) -> DockSize {
        self.thickness
    }

    fn length(&self) -> f32 {
        self.length
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }
}

/// The builder for the Toolbar widget
#[derive(Clone, Default)]
pub struct ToolbarBuilder {
    id:          Id,
    thickness:   DockSize,
    orientation: Orientation,
    color:       String,
    parent_id:   Id,
    visible:     bool,
}

impl ToolbarBuilder {
    /// Initialize the builder for the Bar widget
    pub fn new<V>(id: V, parent_id: V) -> Self
    where
        V: Into<Id>,
    {
        Self {
            id: id.into(),
            parent_id: parent_id.into(),
            visible: true,
            ..Self::default()
        }
    }

    /// Initialize the builder for the Bar widget from another Bar widget
    pub fn new_from_toolbar(toolbar: &Toolbar) -> Self {
        Self {
            id:          toolbar.id(),
            thickness:   toolbar.thickness(),
            orientation: toolbar.orientation(),
            color:       toolbar.color().into_hex(),
            parent_id:   toolbar.parent_id().unwrap(),
            visible:     toolbar.visible(),
        }
    }

    /// Set the orientation
    pub fn with_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }

    /// Set the color
    pub fn with_color<V: Into<String>>(&mut self, color: V) -> &mut Self {
        self.color = color.into();
        self
    }

    /// Set the dock size
    pub fn with_thickness(&mut self, thickness: DockSize) -> &mut Self {
        self.thickness = thickness;
        self
    }

    /// Build the Bar widget
    pub fn build<'a>(&self, ui: &'a mut Ui) -> Result<Box<Toolbar>, Error> {
        let thickness = self.thickness;
        let length = 100.0;
        let orientation = self.orientation;
        let (size, position) = match orientation {
            Orientation::Top => (Size::Size(length, thickness.into_f32()), Position::Top),
            Orientation::Bottom => (Size::Size(length, thickness.into_f32()), Position::Bottom),
            Orientation::Left => (Size::Size(thickness.into_f32(), length), Position::Left),
            Orientation::Right => (Size::Size(thickness.into_f32(), length), Position::Right),
        };

        let color = if self.color.is_empty() {
            ui.theme().bar_color()?
        } else {
            Color::from_hex(self.clone().color)?
        };

        let widget = Box::new(Toolbar {
            id: self.clone().id,
            parent_id: self.clone().parent_id,
            position,
            size,
            color,
            visible: self.visible,
            thickness,
            length,
            orientation,
        });

        ui.add_widget(widget.clone());

        Ok(widget)
    }
}
