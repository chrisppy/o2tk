// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate o2tk;

use o2tk::{
    prelude::*,
    widgets::{
        window::{
            dpi::LogicalSize,
            Event,
            WindowEvent,
        },
        ContainerBuilder,
        DockBuilder,
        ToolbarBuilder,
        WindowContainerBuilder,
    },
    Error,
    Ui,
};

fn main() -> Result<(), Error> {
    let ui = Ui::new("org.red-oxide.o2tk-demo");

    let wcontainer = WindowContainerBuilder::new("wcontainer")
        .with_title("O2TK Demo")
        .with_dimensions(LogicalSize::new(800.0, 600.0))
        .with_min_dimensions(LogicalSize::new(800.0, 600.0))
        .build(&ui)?;

    let toolbar = ToolbarBuilder::new("toolbar", &wcontainer.id()).build(&ui)?;

    let container = ContainerBuilder::new("container", &wcontainer.id(), Position::Center).build(&ui)?;

    let child = ContainerBuilder::new("child", &container.id(), Position::Left).build(&ui)?;

    let sec_child = ContainerBuilder::new("sec_child", &container.id(), Position::Right).build(&ui)?;

    let dock = DockBuilder::new("dock", &wcontainer.id())
        .with_orientation(Orientation::Bottom)
        .with_length(50.0)
        .build(&ui)?;

    ui.add_widget(wcontainer.clone())
        .add_widget(container)
        .add_widget(child)
        .add_widget(sec_child)
        .add_widget(toolbar)
        .add_widget(dock)
        .run(&wcontainer, |event, _window| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => Run::Done,
            Event::WindowEvent {
                event: WindowEvent::Resized(..),
                ..
            } => Run::Redraw,
            _ => Run::Continue,
        })?;

    Ok(())
}
