// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use self::super::super::{
    prelude::*,
    Error,
    Ui,
    Uuid,
};

/// The Container Widget
#[derive(Clone)]
pub struct Container {
    id:        Uuid,
    parent_id: Uuid,
    size:      Size,
    position:  Position,
    color:     Color,
    visible:   bool,
}

impl Widget for Container {
    fn widget_type(&self) -> WidgetType {
        WidgetType::Container
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.parent_id)
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

/// The builder for the Container widget
#[derive(Clone, Default)]
pub struct ContainerBuilder {
    size:      Size,
    position:  Position,
    color:     String,
    parent_id: Uuid,
    visible:   bool,
}

impl ContainerBuilder {
    /// Initialize the builder for the Container widget
    pub fn new(parent_id: Uuid, position: Position) -> Self {
        Self {
            position,
            parent_id,
            visible: true,
            ..Self::default()
        }
    }

    /// Initialize the builder for the Container widget from another Container widget
    pub fn new_from_container(container: &Container) -> Self {
        Self {
            size:      container.size(),
            position:  container.position(),
            color:     container.color().into_string(),
            parent_id: container.parent_id().unwrap(),
            visible:   container.visible(),
        }
    }

    /// Set the color
    pub fn with_color<V: Into<String>>(&mut self, color: V) -> &mut Self {
        self.color = color.into();
        self
    }

    /// Set the size
    pub fn with_size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }

    /// Build the Container widget
    pub fn build(&self, ui: &Ui) -> Result<Box<Container>, Error> {
        let color = if self.color.is_empty() {
            Color::from_hex(ui.theme().container_color())?
        } else {
            Color::from_hex(self.clone().color)?
        };

        Ok(Box::new(Container {
            id: ui.gen_id(),
            parent_id: self.parent_id,
            position: self.position,
            size: self.size,
            color,
            visible: self.visible,
        }))
    }
}
