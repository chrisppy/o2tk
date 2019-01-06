// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use self::super::super::prelude::{
    err_msg,
    Color,
    DrawVertex,
    Error,
    Id,
    Position,
    Size,
    Ui,
    WidgetType,
};
use downcast_rs::{
    impl_downcast,
    Downcast,
};

#[derive(Debug, Default, Clone, Copy)]
struct DObject {
    color:     Color,
    tr_vertex: Vertex,
    tl_vertex: Vertex,
    bl_vertex: Vertex,
    br_vertex: Vertex,
}

impl DObject {
    fn color(&self) -> Color {
        self.color
    }

    fn tr_vertex(&self) -> Vertex {
        self.tr_vertex
    }

    fn tl_vertex(&self) -> Vertex {
        self.tl_vertex
    }

    fn bl_vertex(&self) -> Vertex {
        self.bl_vertex
    }

    fn br_vertex(&self) -> Vertex {
        self.br_vertex
    }
}

#[derive(Debug, Default, Clone)]
struct DObjectBuilder {
    color:     Color,
    tr_vertex: Vertex,
    bl_vertex: Vertex,
}

impl DObjectBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    fn with_bl_vertex(&mut self, vertex: Vertex) -> &mut Self {
        self.bl_vertex = vertex;
        self
    }

    fn with_tr_vertex(&mut self, vertex: Vertex) -> &mut Self {
        self.tr_vertex = vertex;
        self
    }

    fn build(&self) -> DObject {
        let color = self.clone().color;
        let bl_vertex = self.bl_vertex;
        let tr_vertex = self.tr_vertex;
        let br_vertex = VertexBuilder::new().with_x(tr_vertex.x()).with_y(bl_vertex.y()).build();
        let tl_vertex = VertexBuilder::new().with_x(bl_vertex.x()).with_y(tr_vertex.y()).build();
        DObject {
            color,
            tr_vertex,
            tl_vertex,
            bl_vertex,
            br_vertex,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Vertex {
    x: f32,
    y: f32,
}

impl Vertex {
    fn x(self) -> f32 {
        self.x
    }

    fn y(self) -> f32 {
        self.y
    }

    fn as_array(self) -> [f32; 2] {
        [self.x - 1.0, self.y - 1.0]
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct VertexBuilder {
    x: f32,
    y: f32,
}

impl VertexBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    fn with_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    fn build(self) -> Vertex {
        let x = self.x;
        let y = self.y;
        Vertex { x, y }
    }
}

fn calc_vertices(bl_vertex: Vertex, tr_vertex: Vertex, size: Size, position: Position) -> (Vertex, Vertex) {
    match size {
        Size::Full => (bl_vertex, tr_vertex),
        Size::Size(xp, yp) => {
            let x = xp / 100.0;
            let y = yp / 100.0;
            match position {
                Position::BottomLeft => (
                    VertexBuilder::new()
                        .with_x(bl_vertex.x())
                        .with_y(tr_vertex.y() - ((tr_vertex.y() - bl_vertex.y()) * y))
                        .build(),
                    VertexBuilder::new()
                        .with_x((tr_vertex.x() - bl_vertex.x()) * x)
                        .with_y(tr_vertex.y())
                        .build(),
                ),
                Position::Bottom => (
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) - ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y(tr_vertex.y() - ((tr_vertex.y() - bl_vertex.y()) * y))
                        .build(),
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) + ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y(tr_vertex.y())
                        .build(),
                ),
                Position::BottomRight => (
                    VertexBuilder::new()
                        .with_x(tr_vertex.x() - ((tr_vertex.x() - bl_vertex.x()) * x))
                        .with_y(tr_vertex.y() - ((tr_vertex.y() - bl_vertex.y()) * y))
                        .build(),
                    tr_vertex,
                ),
                Position::Left => (
                    VertexBuilder::new()
                        .with_x(bl_vertex.x())
                        .with_y(((tr_vertex.y() - bl_vertex.y()) - ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                    VertexBuilder::new()
                        .with_x((tr_vertex.x() - bl_vertex.x()) * x)
                        .with_y(((tr_vertex.y() - bl_vertex.y()) + ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                ),
                Position::Center => (
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) - ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y(((tr_vertex.y() - bl_vertex.y()) - ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) + ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y(((tr_vertex.y() - bl_vertex.y()) + ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                ),
                Position::Right => (
                    VertexBuilder::new()
                        .with_x(tr_vertex.x() - ((tr_vertex.x() - bl_vertex.x()) * x))
                        .with_y(((tr_vertex.y() - bl_vertex.y()) - ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                    VertexBuilder::new()
                        .with_x(tr_vertex.x())
                        .with_y(((tr_vertex.y() - bl_vertex.y()) + ((tr_vertex.y() - bl_vertex.y()) * y)) / 2.0)
                        .build(),
                ),
                Position::TopLeft => (
                    bl_vertex,
                    VertexBuilder::new()
                        .with_x((tr_vertex.x() - bl_vertex.x()) * x)
                        .with_y((tr_vertex.y() - bl_vertex.y()) * y)
                        .build(),
                ),
                Position::Top => (
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) - ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y(bl_vertex.y())
                        .build(),
                    VertexBuilder::new()
                        .with_x(((tr_vertex.x() - bl_vertex.x()) + ((tr_vertex.x() - bl_vertex.x()) * x)) / 2.0)
                        .with_y((tr_vertex.y() - bl_vertex.y()) * y)
                        .build(),
                ),
                Position::TopRight => (
                    VertexBuilder::new()
                        .with_x(tr_vertex.x() - ((tr_vertex.x() - bl_vertex.x()) * x))
                        .with_y(bl_vertex.y())
                        .build(),
                    VertexBuilder::new()
                        .with_x(tr_vertex.x())
                        .with_y((tr_vertex.y() - bl_vertex.y()) * y)
                        .build(),
                ),
            }
        }
    }
}

/// The Trait that all widgets must implement
pub trait WidgetTrait: WidgetClone + Downcast {
    /// The type of this widget
    fn widget_type(&self) -> WidgetType;
    /// The id of this widget
    fn id(&self) -> Id;
    /// The id of the parent of this widget, if one exist, and it will be None for Window Container
    fn parent_id(&self) -> Option<Id>;
    /// Retrieve the size of the this widget in the format (width, height)
    fn size(&self) -> Size;
    /// Set the size of the this widget in the format (width, height)
    fn set_size(&mut self, size: Size);
    /// Retrieve the position of this widget
    fn position(&self) -> Position;
    /// Retrieve the color of this widget
    fn color(&self) -> Color;
    /// The visibility of thi widget
    fn visible(&self) -> bool;
    /// Make this widget visible
    fn show(&mut self);
    /// Hide this widget
    fn hide(&mut self);
    /// Calculate and retrieve the vertices of this widget
    fn draw(&self, ui: &Ui) -> Result<Vec<DrawVertex>, Error> {
        let mut vertices = Vec::new();
        if !self.visible() {
            return Ok(vertices);
        }

        let mut bl_vertex = VertexBuilder::new().with_x(0.0).with_y(0.0).build();
        let mut tr_vertex = VertexBuilder::new().with_x(2.0).with_y(2.0).build();

        let mut parent_id = self.parent_id();
        let size = self.size();

        let position = self.position();
        let color = self.color();
        let mut parents: Vec<Id> = Vec::new();

        loop {
            match parent_id {
                None => {
                    break;
                }
                Some(pid) => match ui.widgets().get(&pid) {
                    None => {
                        return Err(err_msg("DVError1: Attempted to use an id that does not exist"));
                    }
                    Some(widget) => {
                        let widget = widget.lock();
                        parents.push(pid);
                        parent_id = widget.parent_id();
                    }
                },
            }
        }

        let mut parent_vertices = (bl_vertex, tr_vertex);
        for parent in parents.iter().rev() {
            let bl_vertex = parent_vertices.0;
            let tr_vertex = parent_vertices.1;

            match ui.widgets().get(parent) {
                None => {
                    return Err(err_msg("DVError2: Attempted to use an id that does not exist"));
                }
                Some(widget) => {
                    let widget = widget.lock();
                    if !widget.visible() {
                        return Ok(vertices);
                    }
                    parent_vertices = calc_vertices(bl_vertex, tr_vertex, widget.size(), widget.position());
                }
            }
        }

        let bl = parent_vertices.0;
        let tr = parent_vertices.1;
        parent_vertices = calc_vertices(bl, tr, size, position);

        bl_vertex = parent_vertices.0;
        tr_vertex = parent_vertices.1;

        let dobject = DObjectBuilder::new()
            .with_bl_vertex(bl_vertex)
            .with_tr_vertex(tr_vertex)
            .with_color(color)
            .build();

        let tr_vertex = dobject.tr_vertex().as_array();
        let tl_vertex = dobject.tl_vertex().as_array();
        let bl_vertex = dobject.bl_vertex().as_array();
        let br_vertex = dobject.br_vertex().as_array();
        let color = dobject.color().into_scaled_rgba_float();

        vertices.push(DrawVertex {
            position: tr_vertex,
            color,
        });
        vertices.push(DrawVertex {
            position: tl_vertex,
            color,
        });
        vertices.push(DrawVertex {
            position: bl_vertex,
            color,
        });
        vertices.push(DrawVertex {
            position: tr_vertex,
            color,
        });
        vertices.push(DrawVertex {
            position: br_vertex,
            color,
        });
        vertices.push(DrawVertex {
            position: bl_vertex,
            color,
        });

        Ok(vertices)
    }
}

impl_downcast!(WidgetTrait);

/// Allow the Widget Trait to be cloned
pub trait WidgetClone {
    /// Clone the boxed widget
    fn clone_box(&self) -> Box<WidgetTrait>;
}

impl<T> WidgetClone for T
where
    T: 'static + WidgetTrait + Clone,
{
    fn clone_box(&self) -> Box<WidgetTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<WidgetTrait> {
    fn clone(&self) -> Box<WidgetTrait> {
        self.clone_box()
    }
}
