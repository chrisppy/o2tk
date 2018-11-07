// Copyright (C) 2018 red-oxide developers
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
    container_vertices,
};
use o2tk::prelude::*;

#[test]
fn test_container_bottom_left() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(
        expected,
        container_vertices(Size::Size(100.0, 100.0), Position::BottomLeft),
    );
}

#[test]
fn test_container_bottom() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, container_vertices(Size::Size(100.0, 100.0), Position::Bottom));
}

#[test]
fn test_container_bottom_right() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(
        expected,
        container_vertices(Size::Size(100.0, 100.0), Position::BottomRight),
    );
}

#[test]
fn test_container_left() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, container_vertices(Size::Size(100.0, 100.0), Position::Left));
}

#[test]
fn test_container_center() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, container_vertices(Size::Size(100.0, 100.0), Position::Center));
}

#[test]
fn test_container_right() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, container_vertices(Size::Size(100.0, 100.0), Position::Right));
}

#[test]
fn test_container_top_left() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(
        expected,
        container_vertices(Size::Size(100.0, 100.0), Position::TopLeft),
    );
}

#[test]
fn test_container_top() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(expected, container_vertices(Size::Size(100.0, 100.0), Position::Top));
}

#[test]
fn test_container_top_right() {
    let tr = [1.0, 1.0];
    let tl = [-1.0, 1.0];
    let bl = [-1.0, -1.0];
    let br = [1.0, -1.0];
    let expected = vec![tr, tl, bl, tr, br, bl];
    compare(
        expected,
        container_vertices(Size::Size(100.0, 100.0), Position::TopRight),
    );
}
