// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate o2tk;

use o2tk::{
    prelude::*,
    window::{
        dpi::LogicalSize,
        Event,
        WindowEvent,
    },
    ContainerBuilder,
    DockBuilder,
    LabelBuilder,
    ToolbarBuilder,
    WindowContainerBuilder,
};

fn main() -> Result<(), Error> {
    let mut ui = Ui::init("org.red-oxide.o2tk-demo")?;

    let wcontainer = WindowContainerBuilder::new("wcontainer")
        .with_title("O2TK Demo")
        .with_dimensions(LogicalSize::new(800.0, 600.0))
        .with_min_dimensions(LogicalSize::new(800.0, 600.0))
        .build(&mut ui)?;

    let container = ContainerBuilder::new("container", &wcontainer.id(), Position::Center).build(&mut ui)?;

    let child = ContainerBuilder::new("child", &container.id(), Position::Left).build(&mut ui)?;

    let _label = LabelBuilder::new("label", &child.id(), Position::Center, "Testing")
        .with_size(Size::Size(50.0, 50.0))
        .with_background_color("#123456FF")
        .with_text_color("#FFFFFFFF")
        .build(&mut ui)?;

    let _sec_child = ContainerBuilder::new("sec_child", &container.id(), Position::Right).build(&mut ui)?;

    let _toolbar = ToolbarBuilder::new("toolbar", &wcontainer.id()).build(&mut ui)?;

    let _dock = DockBuilder::new("dock", &wcontainer.id())
        .with_orientation(Orientation::Bottom)
        .with_length(50.0)
        .build(&mut ui)?;

    ui.run(wcontainer.id(), |event, _window| match event {
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
