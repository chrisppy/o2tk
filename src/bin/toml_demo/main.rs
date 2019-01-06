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
        Event,
        WindowEvent,
    },
};

fn main() -> Result<(), Error> {
    let mut ui = Ui::init("org.red-oxide.o2tk-demo")?;

    ui.add_from_file("./src/bin/toml_demo/ui.toml")?;

    ui.run(String::from("wcontainer"), |event, _window| match event {
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
