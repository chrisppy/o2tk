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
pub struct Container {
    id:        Id,
    parent_id: Id,
    size:      Size,
    position:  Position,
    color:     Color,
    visible:   bool,
}

impl WidgetTrait for Container {
    fn widget_type(&self) -> WidgetType {
        WidgetType::Container
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

/// The builder for the Container widget
#[derive(Clone, Default)]
pub struct ContainerBuilder {
    id:        Id,
    size:      Size,
    position:  Position,
    color:     String,
    parent_id: Id,
    visible:   bool,
}

impl ContainerBuilder {
    /// Initialize the builder for the Container widget
    pub fn new<V>(id: V, parent_id: V, position: Position) -> Self
    where
        V: Into<Id>,
    {
        Self {
            id: id.into(),
            position,
            parent_id: parent_id.into(),
            visible: true,
            ..Self::default()
        }
    }

    /// Initialize the builder for the Container widget from another Container widget
    pub fn new_from_container(container: &Container) -> Self {
        Self {
            id:        container.id(),
            size:      container.size(),
            position:  container.position(),
            color:     container.color().into_hex(),
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
    pub fn build<'a>(&self, ui: &'a mut Ui) -> Result<Box<Container>, Error> {
        let color = if self.color.is_empty() {
            ui.theme().container_color()?
        } else {
            Color::from_hex(self.clone().color)?
        };

        let widget = Box::new(Container {
            id: self.clone().id,
            parent_id: self.clone().parent_id,
            position: self.position,
            size: self.size,
            color,
            visible: self.visible,
        });

        ui.add_widget(widget.clone());

        Ok(widget)
    }
}
