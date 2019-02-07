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

/// The Label Widget
#[derive(Clone)]
pub struct Label {
    id:         Id,
    parent_id:  Id,
    size:       Size,
    position:   Position,
    bg_color:   Color,
    visible:    bool,
    label:      String,
    text_color: Color,
    text_size:  f32,
}

impl WidgetTrait for Label {
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
        self.bg_color
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

impl TextTrait for Label {
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

/// The builder for the Label widget
#[derive(Clone, Default)]
pub struct LabelBuilder {
    id:         Id,
    size:       Size,
    position:   Position,
    bg_color:   String,
    text_color: String,
    parent_id:  Id,
    visible:    bool,
    label:      String,
    text_size:  f32,
}

impl LabelBuilder {
    /// Initialize the builder for the Label widget
    pub fn new<S, V>(id: V, parent_id: V, position: Position, label: S) -> Self
    where
        S: Into<String>,
        V: Into<Id>,
    {
        Self {
            id: id.into(),
            position,
            parent_id: parent_id.into(),
            visible: true,
            label: label.into(),
            text_size: 100.0,
            ..Self::default()
        }
    }

    /// Initialize the builder for the Label widget from another Label widget
    pub fn new_from_label(label: &Label) -> Self {
        Self {
            id:         label.id(),
            size:       label.size(),
            position:   label.position(),
            bg_color:   label.color().into_hex(),
            text_color: label.text_color().into_hex(),
            parent_id:  label.parent_id().unwrap(),
            visible:    label.visible(),
            label:      label.label(),
            text_size:  label.text_size(),
        }
    }

    /// Set the color
    pub fn with_background_color<V: Into<String>>(&mut self, bg_color: V) -> &mut Self {
        self.bg_color = bg_color.into();
        self
    }

    /// Set the color
    pub fn with_text_color<V: Into<String>>(&mut self, text_color: V) -> &mut Self {
        self.text_color = text_color.into();
        self
    }

    /// Set the size
    pub fn with_size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the size
    pub fn with_text_size(&mut self, text_size: f32) -> &mut Self {
        self.text_size = text_size;
        self
    }

    /// Build the Label widget
    pub fn build<'a>(&self, ui: &'a mut Ui) -> Result<Box<Label>, Error> {
        let bg_color = if self.bg_color.is_empty() {
            ui.theme().label_background_color()?
        } else {
            Color::from_hex(self.clone().bg_color)?
        };

        let text_color = if self.text_color.is_empty() {
            ui.theme().label_text_color()?
        } else {
            Color::from_hex(self.clone().text_color)?
        };

        if self.text_size <= 0.0 {
            return Err(err_msg("The size must be greater than 0"));
        } else if self.text_size > 100.0 {
            return Err(err_msg("The size must be 100 or less"));
        }

        let widget = Box::new(Label {
            id: self.clone().id,
            parent_id: self.clone().parent_id,
            position: self.position,
            size: self.size,
            bg_color,
            text_color,
            visible: self.visible,
            label: self.clone().label,
            text_size: self.text_size,
        });

        ui.add_widget(widget.clone());

        Ok(widget)
    }
}
