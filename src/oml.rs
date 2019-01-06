// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

#![deny(missing_docs)]

use self::super::{
    prelude::*,
    window::dpi::LogicalSize,
    ContainerBuilder,
    DockBuilder,
    LabelBuilder,
    ToolbarBuilder,
    WindowContainerBuilder,
};
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::Path,
};
use toml::from_slice;

#[derive(Debug, Deserialize, Clone)]
struct Widgets {
    window_container: Option<WindowContainerMarkup>,
    toolbar:          Option<Vec<ToolbarMarkup>>,
    dock:             Option<Vec<DockMarkup>>,
    container:        Option<Vec<ContainerMarkup>>,
    label:            Option<Vec<LabelMarkup>>,
}

#[derive(Debug, Deserialize, Clone)]
struct WindowContainerMarkup {
    id:             Id,
    title:          String,
    position:       Option<Position>,
    color:          Option<String>,
    dimensions:     Option<(f64, f64)>,
    min_dimensions: Option<(f64, f64)>,
    max_dimensions: Option<(f64, f64)>,
    resizable:      Option<bool>,
    maximized:      Option<bool>,
    visible:        Option<bool>,
    transparent:    Option<bool>,
    decorations:    Option<bool>,
    always_on_top:  Option<bool>,
    multitouch:     Option<bool>,
    toolbar:        Option<Vec<ToolbarMarkup>>,
    container:      Option<Vec<ContainerMarkup>>,
    dock:           Option<Vec<DockMarkup>>,
    label:          Option<Vec<LabelMarkup>>,
}

#[derive(Debug, Deserialize, Clone)]
struct ToolbarMarkup {
    id:          Id,
    thickness:   Option<DockSize>,
    orientation: Option<Orientation>,
    color:       Option<String>,
    parent_id:   Option<Id>,
    visible:     Option<bool>,
    toolbar:     Option<Vec<ToolbarMarkup>>,
    container:   Option<Vec<ContainerMarkup>>,
    dock:        Option<Vec<DockMarkup>>,
    label:       Option<Vec<LabelMarkup>>,
}

#[derive(Debug, Deserialize, Clone)]
struct ContainerMarkup {
    id:        Id,
    position:  Position,
    size:      Option<Size>,
    color:     Option<String>,
    parent_id: Option<Id>,
    visible:   Option<bool>,
    toolbar:   Option<Vec<ToolbarMarkup>>,
    container: Option<Vec<ContainerMarkup>>,
    dock:      Option<Vec<DockMarkup>>,
    label:     Option<Vec<LabelMarkup>>,
}

#[derive(Debug, Deserialize, Clone)]
struct DockMarkup {
    id:          Id,
    length:      Option<f32>,
    thickness:   Option<DockSize>,
    orientation: Option<Orientation>,
    color:       Option<String>,
    parent_id:   Option<Id>,
    visible:     Option<bool>,
    toolbar:     Option<Vec<ToolbarMarkup>>,
    container:   Option<Vec<ContainerMarkup>>,
    dock:        Option<Vec<DockMarkup>>,
    label:       Option<Vec<LabelMarkup>>,
}

#[derive(Debug, Deserialize, Clone)]
struct LabelMarkup {
    id:               Id,
    text:             String,
    position:         Position,
    size:             Option<Size>,
    background_color: Option<String>,
    text_color:       Option<String>,
    text_size:        Option<f32>,
    parent_id:        Option<Id>,
    visible:          Option<bool>,
    toolbar:          Option<Vec<ToolbarMarkup>>,
    container:        Option<Vec<ContainerMarkup>>,
    dock:             Option<Vec<DockMarkup>>,
    label:            Option<Vec<LabelMarkup>>,
}

fn from_reader<R, T>(r: &mut R) -> Result<T, Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;

    match from_slice(&buf) {
        Err(err) => {
            return Err(err_msg(format!("Failed to load ui from slice: {}", err)));
        }
        Ok(val) => Ok(val),
    }
}

/// Trait to allow building the UI by means of a toml file
pub trait MlBuild {
    /// Add widgets by means of a markup in toml
    fn add_from_file<'a>(&'a mut self, path: &str) -> Result<&'a mut Ui, Error>;
}

impl MlBuild for Ui {
    fn add_from_file<'a>(&'a mut self, path: &str) -> Result<&'a mut Self, Error> {
        let p = Path::new(path);

        match p.extension() {
            None => {
                return Err(err_msg("Error with the file name"));
            }
            Some(v) => match v.to_str() {
                None => {
                    return Err(err_msg("Incorrect file extension, must be toml"));
                }
                Some(v) => {
                    if (v.len() != 4) | !v.contains("toml") {
                        return Err(err_msg("Incorrect file extension, must be toml"));
                    }
                }
            },
        }

        let mut f = match File::open(p) {
            Err(err) => {
                return Err(err_msg(format!("Failed opening toml file: {}", err)));
            }
            Ok(val) => val,
        };

        let widgets: Widgets = match from_reader(&mut f) {
            Err(err) => {
                return Err(err_msg(format!("Failed to load widgets: {}", err)));
            }
            Ok(val) => val,
        };

        if let Some(widget) = widgets.window_container {
            add_window_container(self, widget)?;
        }

        if let Some(widget) = widgets.container {
            for container in widget {
                add_container(self, container, None)?;
            }
        }

        if let Some(widget) = widgets.toolbar {
            for toolbar in widget {
                add_toolbar(self, toolbar, None)?;
            }
        }

        if let Some(widget) = widgets.dock {
            for dock in widget {
                add_dock(self, dock, None)?;
            }
        }

        if let Some(widget) = widgets.label {
            for label in widget {
                add_label(self, label, None)?;
            }
        }

        Ok(self)
    }
}

fn add_window_container<'a>(ui: &'a mut Ui, widget: WindowContainerMarkup) -> Result<&'a mut Ui, Error> {
    let id = widget.id;
    let mut builder = WindowContainerBuilder::new(id.clone());
    builder.with_title(widget.title);

    if let Some(val) = widget.dimensions {
        builder.with_dimensions(LogicalSize::new(val.0, val.1));
    }
    if let Some(val) = widget.max_dimensions {
        builder.with_max_dimensions(LogicalSize::new(val.0, val.1));
    }
    if let Some(val) = widget.min_dimensions {
        builder.with_min_dimensions(LogicalSize::new(val.0, val.1));
    }
    if let Some(val) = widget.color {
        builder.with_color(val);
    }
    if let Some(val) = widget.resizable {
        builder.with_resizable(val);
    }
    if let Some(val) = widget.maximized {
        builder.with_maximized(val);
    }
    if let Some(val) = widget.maximized {
        builder.with_maximized(val);
    }
    if let Some(val) = widget.visible {
        builder.with_visibility(val);
    }
    if let Some(val) = widget.transparent {
        builder.with_transparency(val);
    }
    if let Some(val) = widget.decorations {
        builder.with_decorations(val);
    }
    if let Some(val) = widget.always_on_top {
        builder.with_always_on_top(val);
    }
    if let Some(val) = widget.multitouch {
        if val {
            builder.with_multitouch();
        }
    }

    builder.build(ui)?;

    if let Some(widget) = widget.container {
        for container in widget {
            add_container(ui, container, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.toolbar {
        for toolbar in widget {
            add_toolbar(ui, toolbar, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.dock {
        for dock in widget {
            add_dock(ui, dock, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.label {
        for label in widget {
            add_label(ui, label, Some(id.clone()))?;
        }
    }

    Ok(ui)
}

fn add_container<'a>(ui: &'a mut Ui, widget: ContainerMarkup, parent_id: Option<Id>) -> Result<&'a mut Ui, Error> {
    let id = widget.id;
    let parent_id = match parent_id {
        Some(val) => val,
        None => match widget.parent_id {
            None => {
                return Err(err_msg(
                    "The parent id is missing in either the text itself, or by means of nesting the widgets",
                ));
            }
            Some(val) => val,
        },
    };

    let mut builder = ContainerBuilder::new(id.clone(), parent_id, widget.position);

    if let Some(val) = widget.color {
        builder.with_color(val);
    }
    if let Some(val) = widget.size {
        builder.with_size(val);
    }

    builder.build(ui)?;

    if let Some(widget) = widget.container {
        for container in widget {
            add_container(ui, container, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.toolbar {
        for toolbar in widget {
            add_toolbar(ui, toolbar, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.dock {
        for dock in widget {
            add_dock(ui, dock, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.label {
        for label in widget {
            add_label(ui, label, Some(id.clone()))?;
        }
    }

    Ok(ui)
}

fn add_toolbar<'a>(ui: &'a mut Ui, widget: ToolbarMarkup, parent_id: Option<Id>) -> Result<&'a mut Ui, Error> {
    let id = widget.id;
    let parent_id = match parent_id {
        Some(val) => val,
        None => match widget.parent_id {
            None => {
                return Err(err_msg(
                    "The parent id is missing in either the text itself, or by means of nesting the widgets",
                ));
            }
            Some(val) => val,
        },
    };

    let mut builder = ToolbarBuilder::new(id.clone(), parent_id);

    if let Some(val) = widget.color {
        builder.with_color(val);
    }
    if let Some(val) = widget.orientation {
        builder.with_orientation(val);
    }
    if let Some(val) = widget.thickness {
        builder.with_thickness(val);
    }

    builder.build(ui)?;

    if let Some(widget) = widget.container {
        for container in widget {
            add_container(ui, container, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.toolbar {
        for toolbar in widget {
            add_toolbar(ui, toolbar, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.dock {
        for dock in widget {
            add_dock(ui, dock, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.label {
        for label in widget {
            add_label(ui, label, Some(id.clone()))?;
        }
    }

    Ok(ui)
}

fn add_dock<'a>(ui: &'a mut Ui, widget: DockMarkup, parent_id: Option<Id>) -> Result<&'a mut Ui, Error> {
    let id = widget.id;
    let parent_id = match parent_id {
        Some(val) => val,
        None => match widget.parent_id {
            None => {
                return Err(err_msg(
                    "The parent id is missing in either the text itself, or by means of nesting the widgets",
                ));
            }
            Some(val) => val,
        },
    };

    let mut builder = DockBuilder::new(id.clone(), parent_id);

    if let Some(val) = widget.color {
        builder.with_color(val);
    }
    if let Some(val) = widget.orientation {
        builder.with_orientation(val);
    }
    if let Some(val) = widget.thickness {
        builder.with_thickness(val);
    }
    if let Some(val) = widget.length {
        builder.with_length(val);
    }

    builder.build(ui)?;

    if let Some(widget) = widget.container {
        for container in widget {
            add_container(ui, container, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.toolbar {
        for toolbar in widget {
            add_toolbar(ui, toolbar, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.dock {
        for dock in widget {
            add_dock(ui, dock, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.label {
        for label in widget {
            add_label(ui, label, Some(id.clone()))?;
        }
    }

    Ok(ui)
}

fn add_label<'a>(ui: &'a mut Ui, widget: LabelMarkup, parent_id: Option<Id>) -> Result<&'a mut Ui, Error> {
    let id = widget.id;
    let parent_id = match parent_id {
        Some(val) => val,
        None => match widget.parent_id {
            None => {
                return Err(err_msg(
                    "The parent id is missing in either the text itself, or by means of nesting the widgets",
                ));
            }
            Some(val) => val,
        },
    };

    let mut builder = LabelBuilder::new(id.clone(), parent_id, widget.position, widget.text);

    if let Some(val) = widget.background_color {
        builder.with_background_color(val);
    }
    if let Some(val) = widget.text_color {
        builder.with_text_color(val);
    }
    if let Some(val) = widget.size {
        builder.with_size(val);
    }
    if let Some(val) = widget.text_size {
        builder.with_text_size(val);
    }

    builder.build(ui)?;

    if let Some(widget) = widget.container {
        for container in widget {
            add_container(ui, container, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.toolbar {
        for toolbar in widget {
            add_toolbar(ui, toolbar, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.dock {
        for dock in widget {
            add_dock(ui, dock, Some(id.clone()))?;
        }
    }

    if let Some(widget) = widget.label {
        for label in widget {
            add_label(ui, label, Some(id.clone()))?;
        }
    }

    Ok(ui)
}
