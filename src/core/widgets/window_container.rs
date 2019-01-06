// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use crate::{
    prelude::*,
    window::{
        dpi::LogicalSize,
        MonitorId,
        WindowBuilder,
    },
};

/// The most top level widget
#[derive(Clone)]
pub struct WindowContainer {
    window: Box<WindowBuilder>,
    id:     Id,
    color:  Color,
}

impl WidgetTrait for WindowContainer {
    fn widget_type(&self) -> WidgetType {
        WidgetType::WindowContainer
    }

    fn id(&self) -> Id {
        self.clone().id
    }

    fn parent_id(&self) -> Option<Id> {
        None
    }

    fn size(&self) -> Size {
        Size::Size(100.0, 100.0)
    }

    fn set_size(&mut self, _size: Size) {
        panic!("You cannot call this method for WindowContainers")
    }

    fn position(&self) -> Position {
        Position::Center
    }

    fn color(&self) -> Color {
        self.color
    }

    fn visible(&self) -> bool {
        true
    }

    fn show(&mut self) {
        panic!("Cannot call show for a window container")
    }

    fn hide(&mut self) {
        panic!("Cannot call hide for a window container")
    }
}

impl WindowContainer {
    /// Retrieve the application window
    pub fn window(&self) -> Box<WindowBuilder> {
        self.clone().window
    }
}

/// The builder for the Window Container widget
#[derive(Clone)]
pub struct WindowContainerBuilder {
    id:             Id,
    position:       Position,
    color:          String,
    dimensions:     Option<LogicalSize>,
    min_dimensions: Option<LogicalSize>,
    max_dimensions: Option<LogicalSize>,
    resizable:      bool,
    fullscreen:     Option<MonitorId>,
    title:          String,
    maximized:      bool,
    visible:        bool,
    transparent:    bool,
    decorations:    bool,
    always_on_top:  bool,
    multitouch:     bool,
}

impl Default for WindowContainerBuilder {
    fn default() -> Self {
        Self {
            id:             String::new(),
            position:       Position::Center,
            color:          String::new(),
            dimensions:     None,
            min_dimensions: None,
            max_dimensions: None,
            resizable:      true,
            title:          "o2tk window".to_owned(),
            maximized:      true,
            fullscreen:     None,
            visible:        true,
            transparent:    false,
            decorations:    true,
            always_on_top:  false,
            multitouch:     false,
        }
    }
}

impl WindowContainerBuilder {
    /// Initialize the builder for the WindowContainer widget
    pub fn new<V>(id: V) -> Self
    where
        V: Into<Id>,
    {
        Self {
            id: id.into(),
            ..Self::default()
        }
    }

    /// Set the color
    pub fn with_color<'a, V: Into<String>>(&'a mut self, color: V) -> &'a mut Self {
        self.color = color.into();
        self
    }

    /// Requests the window to be of specific dimensions.
    pub fn with_dimensions<'a>(&'a mut self, size: LogicalSize) -> &'a mut Self {
        self.dimensions = Some(size);
        self
    }

    /// Sets a minimum dimension size for the window
    pub fn with_min_dimensions<'a>(&'a mut self, min_size: LogicalSize) -> &'a mut Self {
        self.min_dimensions = Some(min_size);
        self
    }

    /// Sets a maximum dimension size for the window
    pub fn with_max_dimensions<'a>(&'a mut self, max_size: LogicalSize) -> &'a mut Self {
        self.max_dimensions = Some(max_size);
        self
    }

    /// Sets whether the window is resizable or not
    ///
    /// Note that making the window unresizable doesn't exempt you from handling `Resized`, as that
    /// event can still be triggered by DPI scaling, entering fullscreen mode, etc.
    ///
    /// ## Platform-specific
    ///
    /// This only has an effect on desktop platforms.
    ///
    /// Due to a bug in XFCE, this has no effect on Xfwm.
    pub fn with_resizable<'a>(&'a mut self, resizable: bool) -> &'a mut Self {
        self.resizable = resizable;
        self
    }

    /// Requests a specific title for the window.
    pub fn with_title<'a, T: Into<String>>(&'a mut self, title: T) -> &'a mut Self {
        self.title = title.into();
        self
    }

    /// Sets the window fullscreen state. None means a normal window, Some(MonitorId)
    /// means a fullscreen window on that specific monitor
    pub fn with_fullscreen<'a>(&'a mut self, monitor: Option<MonitorId>) -> &'a mut Self {
        self.fullscreen = monitor;
        self
    }

    /// Requests maximized mode.
    pub fn with_maximized<'a>(&'a mut self, maximized: bool) -> &'a mut Self {
        self.maximized = maximized;
        self
    }

    /// Sets whether the window will be initially hidden or visible.
    pub fn with_visibility<'a>(&'a mut self, visible: bool) -> &'a mut Self {
        self.visible = visible;
        self
    }

    /// Sets whether the background of the window should be transparent.
    pub fn with_transparency<'a>(&'a mut self, transparent: bool) -> &'a mut Self {
        self.transparent = transparent;
        self
    }

    /// Sets whether the window should have a border, a title bar, etc.
    pub fn with_decorations<'a>(&'a mut self, decorations: bool) -> &'a mut Self {
        self.decorations = decorations;
        self
    }

    /// Sets whether or not the window will always be on top of other windows.
    pub fn with_always_on_top<'a>(&'a mut self, always_on_top: bool) -> &'a mut Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Enables multitouch.
    pub fn with_multitouch<'a>(&'a mut self) -> &'a mut Self {
        self.multitouch = true;
        self
    }

    /// Build the WindowContainer widget
    pub fn build<'a>(&self, ui: &'a mut Ui) -> Result<Box<WindowContainer>, Error> {
        let id = self.clone().id;

        let color = if self.color.is_empty() {
            ui.theme().window_container_color()?
        } else {
            Color::from_hex(self.clone().color)?
        };

        let w = WindowBuilder::new()
            .with_resizable(self.resizable)
            .with_title(self.clone().title)
            .with_maximized(self.maximized)
            .with_fullscreen(self.clone().fullscreen)
            .with_visibility(self.visible)
            .with_transparency(self.transparent)
            .with_decorations(self.decorations)
            .with_always_on_top(self.always_on_top);

        let x = if let Some(val) = self.dimensions {
            w.with_dimensions(val)
        } else {
            w
        };

        let y = if let Some(val) = self.min_dimensions {
            x.with_min_dimensions(val)
        } else {
            x
        };

        let z = if let Some(val) = self.max_dimensions {
            y.with_max_dimensions(val)
        } else {
            y
        };

        let win = if self.multitouch { z.with_multitouch() } else { z };

        let window = Box::new(win);

        let widget = Box::new(WindowContainer { window, id, color });

        ui.add_widget(widget.clone());

        Ok(widget)
    }
}
