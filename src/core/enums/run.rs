// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

/// The different run statuses of the application
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Run {
    /// The status to end the application
    Done,
    /// The status to redraw the screen when an UI element has changed
    Redraw,
    /// The status to let the application run continue to run when there is no drawing changes
    Continue,
}

impl Default for Run {
    /// The default Run
    fn default() -> Run {
        Run::Continue
    }
}
