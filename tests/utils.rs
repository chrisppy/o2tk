// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

#![allow(dead_code)]

use o2tk::{
    prelude::*,
    widgets::{
        window::dpi::LogicalSize,
        ContainerBuilder,
        DockBuilder,
        ToolbarBuilder,
        WindowContainerBuilder,
    },
    Ui,
};
use std::f32::EPSILON;

static APP_ID: &str = "org.red-oxide.test";

pub fn compare(expected: Vec<[f32; 2]>, actual: Vec<[f32; 2]>) {
    let mut err = String::new();

    if expected.len() != 6 {
        err += "The expected vec size must be 6.\n";
    }

    if actual.len() != 6 {
        err += "The actual vec size must be 6.\n";
    }

    // Check if Equal without the need for the tolerance
    if expected.eq(&actual) {
        return;
    }

    // Check if Equal within tolerance for each vertex
    for i in 0..6 {
        let p = match i {
            0 => "Top Right",
            1 => "Top Left",
            2 => "Bottom Left",
            3 => "Top Right",
            4 => "Bottom Right",
            5 => "Bottom Left",
            _ => "INVALID",
        };

        let e = expected.get(i).unwrap();
        let a = actual.get(i).unwrap();

        if (a[0] > e[0] + EPSILON) | (a[0] < e[0] - EPSILON) | (a[1] > e[1] + EPSILON) | (a[1] < e[1] - EPSILON) {
            err += format!(
                "The {} vertex for actual: {:?} and expected: {:?} does not fall within the EPSILON tolerance.\n",
                p, a, e
            )
            .as_str();
        }
    }

    if !err.is_empty() {
        assert!(false, err);
    }
}

pub fn bar_vertices(orientation: Orientation, size: DockSize) -> Vec<[f32; 2]> {
    let ui = Ui::new(APP_ID);

    let wcontainer = WindowContainerBuilder::new("wcontainer")
        .with_title("O2TK Demo")
        .with_dimensions(LogicalSize::new(800.0, 600.0))
        .with_min_dimensions(LogicalSize::new(800.0, 600.0))
        .build(&ui)
        .unwrap();

    let bar = ToolbarBuilder::new("bar", &wcontainer.id())
        .with_orientation(orientation)
        .with_thickness(size)
        .build(&ui)
        .unwrap();

    let ui = ui.add_widget(wcontainer).add_widget(bar.clone());

    let mut to_return = Vec::new();
    for vertex in bar.draw(&ui).unwrap() {
        to_return.push(vertex.position());
    }
    to_return
}

pub fn container_vertices(size: Size, position: Position) -> Vec<[f32; 2]> {
    let ui = Ui::new(APP_ID);

    let wcontainer = WindowContainerBuilder::new("wcontainer")
        .with_title("O2TK Demo")
        .with_dimensions(LogicalSize::new(800.0, 600.0))
        .with_min_dimensions(LogicalSize::new(800.0, 600.0))
        .build(&ui)
        .unwrap();

    let container = ContainerBuilder::new("container", &wcontainer.id(), position)
        .with_size(size)
        .build(&ui)
        .unwrap();

    let ui = ui.add_widget(wcontainer).add_widget(container.clone());

    let mut to_return = Vec::new();
    for vertex in container.draw(&ui).unwrap() {
        to_return.push(vertex.position());
    }
    to_return
}

pub fn dock_vertices(length: f32, orientation: Orientation, size: DockSize) -> Vec<[f32; 2]> {
    let ui = Ui::new(APP_ID);

    let wcontainer = WindowContainerBuilder::new("wcontainer")
        .with_title("O2TK Demo")
        .with_dimensions(LogicalSize::new(800.0, 600.0))
        .with_min_dimensions(LogicalSize::new(800.0, 600.0))
        .build(&ui)
        .unwrap();

    let dock = DockBuilder::new("dock", &wcontainer.id())
        .with_orientation(orientation)
        .with_thickness(size)
        .with_length(length)
        .build(&ui)
        .unwrap();

    let ui = ui.add_widget(wcontainer).add_widget(dock.clone());

    let mut to_return = Vec::new();
    for vertex in dock.draw(&ui).unwrap() {
        to_return.push(vertex.position());
    }
    to_return
}
