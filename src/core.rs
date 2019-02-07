// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Traits and enumerations necessary for the toolkit.

mod color;
pub mod enums;
pub mod prelude;
mod theme;
pub mod traits;
pub mod widgets;

use self::prelude::*;
use indexmap::IndexMap;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    sync::Arc,
};
use vulkano::instance::Instance;
/// Identifier type for looking up widgets
pub type Id = String;

/// x and y vulkano vertex
#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex {
    x: f32,
    y: f32,
}

impl Vertex {
    /// get x vulkano coordinate in 0.0 to 2.0 format
    pub fn x(self) -> f32 {
        self.x
    }

    /// get y vulkano coordinate in 0.0 to 2.0 format
    pub fn y(self) -> f32 {
        self.y
    }

    /// get x and y in -1.0 to 1.0 format
    pub fn as_array(self) -> [f32; 2] {
        [self.x - 1.0, self.y - 1.0]
    }
}

/// The position and color of the vertices to be drawn
#[derive(Debug, Clone, Copy)]
pub struct DrawVertex {
    position: Vertex,
    color:    [f32; 4],
}

impl DrawVertex {
    /// Initialize a vertex
    pub fn new(position: Vertex, color: [f32; 4]) -> Self {
        Self { position, color }
    }

    /// Retrieve the position of the vertex
    pub fn position(&self) -> Vertex {
        self.position
    }

    /// Retrieve the color of the vertex
    pub fn color(&self) -> [f32; 4] {
        self.color
    }
}

/// The main UI structure
#[derive(Clone)]
pub struct Ui {
    app_id:    Id,
    theme:     Theme,
    instance:  Arc<Instance>,
    heirarchy: HashMap<Id, Vec<Id>>,
    widgets:   IndexMap<Id, Arc<Mutex<Box<WidgetTrait>>>>,
}

impl Ui {
    /// Create a new Ui
    pub fn new(
        app_id: Id,
        theme: Theme,
        instance: Arc<Instance>,
        heirarchy: HashMap<Id, Vec<Id>>,
        widgets: IndexMap<Id, Arc<Mutex<Box<WidgetTrait>>>>,
    ) -> Self {
        Self {
            app_id,
            theme,
            instance,
            heirarchy,
            widgets,
        }
    }

    /// Retrieve all the app_id
    pub fn app_id(&self) -> Id {
        self.clone().app_id
    }

    /// Retrieve the themes set by the api
    pub fn theme(&self) -> Theme {
        self.clone().theme
    }

    /// Retrieve the instance set by the api
    pub fn instance(&self) -> Arc<Instance> {
        Arc::clone(&self.instance)
    }

    /// Retrieve the heirarchy set by the api
    pub fn heirarchy(&self) -> &HashMap<Id, Vec<Id>> {
        &self.heirarchy
    }

    /// Retrieve the heirarchy set by the api
    pub fn heirarchy_mut<'a>(&'a mut self) -> &'a mut HashMap<Id, Vec<Id>> {
        &mut self.heirarchy
    }

    /// Retrieve all the widgets
    pub fn widgets(&self) -> &IndexMap<Id, Arc<Mutex<Box<WidgetTrait>>>> {
        &self.widgets
    }

    /// Retrieve all the widgets
    pub fn widgets_mut<'a>(&'a mut self) -> &'a mut IndexMap<Id, Arc<Mutex<Box<WidgetTrait>>>> {
        &mut self.widgets
    }
}
