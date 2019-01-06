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
    bar_vertices,
    compare,
};
use o2tk::prelude::*;

#[test]
fn test_bar_bottom_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, v];
    let br = [1.0, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Bottom, thickness));
}

#[test]
fn test_bar_left_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [-v, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Left, thickness));
}

#[test]
fn test_bar_top_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, -v];
    let tl = [-1.0, -v];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Top, thickness));
}

#[test]
fn test_bar_right_small() {
    let thickness = DockSize::Small;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [v, 1.0];
    let bl = [v, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Right, thickness));
}

#[test]
fn test_bar_bottom_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, v];
    let br = [1.0, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Bottom, thickness));
}

#[test]
fn test_bar_left_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [-v, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Left, thickness));
}

#[test]
fn test_bar_top_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, -v];
    let tl = [-1.0, -v];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Top, thickness));
}

#[test]
fn test_bar_right_normal() {
    let thickness = DockSize::Normal;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [v, 1.0];
    let bl = [v, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Right, thickness));
}

#[test]
fn test_bar_bottom_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, v];
    let br = [1.0, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Bottom, thickness));
}

#[test]
fn test_bar_left_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [-v, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Left, thickness));
}

#[test]
fn test_bar_top_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, -v];
    let tl = [-1.0, -v];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Top, thickness));
}

#[test]
fn test_bar_right_large() {
    let thickness = DockSize::Large;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [v, 1.0];
    let bl = [v, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Right, thickness));
}

#[test]
fn test_bar_bottom_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, v];
    let br = [1.0, v];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Bottom, thickness));
}

#[test]
fn test_bar_left_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [-v, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [-v, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Left, thickness));
}

#[test]
fn test_bar_top_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, -v];
    let tl = [-1.0, -v];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Top, thickness));
}

#[test]
fn test_bar_right_xlarge() {
    let thickness = DockSize::XLarge;
    let x = (100.0 - thickness.into_f32()) / 100.0;
    let v = (2.0 * x) - 1.0;
    let tr = [1.0, 1.0];
    let tl = [v, 1.0];
    let bl = [v, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, bar_vertices(Orientation::Right, thickness));
}
