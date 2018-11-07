// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

use o2tk::prelude::*;

#[test]
fn test_rust_hex_to_rgba() {
    let color = "#B7410EFF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [183.0, 65.0, 14.0, 255.0];
    assert_eq!(c.clone().into_rgba(), rgba_expected);
}

#[test]
fn test_rust_hex_to_rgba_scale() {
    let color = "#B7410EFF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [183.0 / 255.0, 65.0 / 255.0, 14.0 / 255.0, 255.0 / 255.0];
    assert_eq!(c.clone().into_rgba_scale(), rgba_expected);
}

#[test]
fn test_rust_hex_to_hex() {
    let color = "#B7410EFF";
    let c = Color::from_hex(color).unwrap();
    assert_eq!(c.into_string(), String::from(color));
}

#[test]
fn test_azure_hex_to_rgba() {
    let color = "#007FFFFF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [0.0, 127.0, 255.0, 255.0];
    assert_eq!(c.clone().into_rgba(), rgba_expected);
}

#[test]
fn test_azure_hex_to_rgba_scale() {
    let color = "#007FFFFF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [0.0 / 255.0, 127.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0];
    assert_eq!(c.clone().into_rgba_scale(), rgba_expected);
}

#[test]
fn test_azure_hex_to_hex() {
    let color = "#007FFFFF";
    let c = Color::from_hex(color).unwrap();
    assert_eq!(c.into_string(), String::from(color));
}

#[test]
fn test_chartreuse_hex_to_rgba() {
    let color = "#7FFF00FF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [127.0, 255.0, 0.0, 255.0];
    assert_eq!(c.clone().into_rgba(), rgba_expected);
}

#[test]
fn test_chartreuse_hex_to_rgba_scale() {
    let color = "#7FFF00FF";
    let c = Color::from_hex(color).unwrap();
    let rgba_expected = [127.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0];
    assert_eq!(c.clone().into_rgba_scale(), rgba_expected);
}

#[test]
fn test_chartreuse_hex_to_hex() {
    let color = "#7FFF00FF";
    let c = Color::from_hex(color).unwrap();
    assert_eq!(c.into_string(), String::from(color));
}
