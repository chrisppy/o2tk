// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

mod utils;

use self::utils::{
    compare,
    dock_vertices,
};
use o2tk::prelude::*;

#[test]
fn test_dock_bottom_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, 1.0];
    let tl = [-0.75, 1.0];
    let bl = [-0.75, v];
    let br = [0.75, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Bottom, thickness));
}

#[test]
fn test_dock_left_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 0.75];
    let tl = [-1.0, 0.75];
    let bl = [-1.0, -0.75];
    let br = [-v, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Left, thickness));
}

#[test]
fn test_dock_top_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, -v];
    let tl = [-0.75, -v];
    let bl = [-0.75, -1.0];
    let br = [0.75, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Top, thickness));
}

#[test]
fn test_dock_right_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 0.75];
    let tl = [v, 0.75];
    let bl = [v, -0.75];
    let br = [1.0, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Right, thickness));
}

#[test]
fn test_dock_bottom_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, 1.0];
    let tl = [-0.75, 1.0];
    let bl = [-0.75, v];
    let br = [0.75, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Bottom, thickness));
}

#[test]
fn test_dock_left_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 0.75];
    let tl = [-1.0, 0.75];
    let bl = [-1.0, -0.75];
    let br = [-v, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Left, thickness));
}

#[test]
fn test_dock_top_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, -v];
    let tl = [-0.75, -v];
    let bl = [-0.75, -1.0];
    let br = [0.75, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Top, thickness));
}

#[test]
fn test_dock_right_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 0.75];
    let tl = [v, 0.75];
    let bl = [v, -0.75];
    let br = [1.0, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Right, thickness));
}

#[test]
fn test_dock_bottom_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, 1.0];
    let tl = [-0.75, 1.0];
    let bl = [-0.75, v];
    let br = [0.75, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Bottom, thickness));
}

#[test]
fn test_dock_left_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 0.75];
    let tl = [-1.0, 0.75];
    let bl = [-1.0, -0.75];
    let br = [-v, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Left, thickness));
}

#[test]
fn test_dock_top_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, -v];
    let tl = [-0.75, -v];
    let bl = [-0.75, -1.0];
    let br = [0.75, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Top, thickness));
}

#[test]
fn test_dock_right_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 0.75];
    let tl = [v, 0.75];
    let bl = [v, -0.75];
    let br = [1.0, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Right, thickness));
}

#[test]
fn test_dock_bottom_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, 1.0];
    let tl = [-0.75, 1.0];
    let bl = [-0.75, v];
    let br = [0.75, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Bottom, thickness));
}

#[test]
fn test_dock_left_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 0.75];
    let tl = [-1.0, 0.75];
    let bl = [-1.0, -0.75];
    let br = [-v, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Left, thickness));
}

#[test]
fn test_dock_top_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [0.75, -v];
    let tl = [-0.75, -v];
    let bl = [-0.75, -1.0];
    let br = [0.75, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Top, thickness));
}

#[test]
fn test_dock_right_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 0.75];
    let tl = [v, 0.75];
    let bl = [v, -0.75];
    let br = [1.0, -0.75];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, dock_vertices(75.0, Orientation::Right, thickness));
}
