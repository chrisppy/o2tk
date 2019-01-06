// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::*;

/// The Dock Widget
#[derive(Clone)]
pub struct Dock {
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

impl WidgetTrait for Dock {
    fn widget_type(&self) -> WidgetType {
        WidgetType::Dock
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

impl DockTrait for Dock {
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

/// The builder for the Dock widget
#[derive(Clone, Default)]
pub struct DockBuilder {
    id:          Id,
    thickness:   DockSize,
    length:      f32,
    orientation: Orientation,
    color:       String,
    parent_id:   Id,
    visible:     bool,
}

impl DockBuilder {
    /// Initialize the builder for the Dock widget
    pub fn new<V>(id: V, parent_id: V) -> Self
    where
        V: Into<Id>,
    {
        let length = 50.0;
        Self {
            id: id.into(),
            parent_id: parent_id.into(),
            length,
            visible: true,
            ..Self::default()
        }
    }

    /// Initialize the builder for the Dock widget from another Dock widget
    pub fn new_from_dock(dock: &Dock) -> Self {
        Self {
            id:          dock.id(),
            thickness:   dock.thickness(),
            length:      dock.length(),
            orientation: dock.orientation(),
            color:       dock.color().into_hex(),
            parent_id:   dock.parent_id().unwrap(),
            visible:     dock.visible(),
        }
    }

    /// Set the orientation
    pub fn with_orientation<'a>(&'a mut self, orientation: Orientation) -> &'a mut Self {
        self.orientation = orientation;
        self
    }

    /// Set the color
    pub fn with_color<'a, V: Into<String>>(&'a mut self, color: V) -> &'a mut Self {
        self.color = color.into();
        self
    }

    /// Set the length
    pub fn with_length<'a>(&'a mut self, length: f32) -> &'a mut Self {
        self.length = length;
        self
    }

    /// Set the dock size
    pub fn with_thickness<'a>(&'a mut self, thickness: DockSize) -> &'a mut Self {
        self.thickness = thickness;
        self
    }

    /// Build the Dock widget
    pub fn build<'a>(&self, ui: &'a mut Ui) -> Result<Box<Dock>, Error> {
        let thickness = self.thickness;
        let length = self.length;
        let orientation = self.orientation;
        let (size, position) = match orientation {
            Orientation::Top => (Size::Size(length, thickness.into_f32()), Position::Top),
            Orientation::Bottom => (Size::Size(length, thickness.into_f32()), Position::Bottom),
            Orientation::Left => (Size::Size(thickness.into_f32(), length), Position::Left),
            Orientation::Right => (Size::Size(thickness.into_f32(), length), Position::Right),
        };

        let color = if self.color.is_empty() {
            ui.theme().dock_color()?
        } else {
            Color::from_hex(self.clone().color)?
        };

        let widget = Box::new(Dock {
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
