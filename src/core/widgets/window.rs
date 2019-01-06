// Copyright © 2018-2019 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Re-exported methods for windows from the winit library

pub use winit::{
    dpi,
    AvailableMonitorsIter,
    AxisId,
    ButtonId,
    ControlFlow,
    CreationError as WindowCreationError,
    DeviceEvent,
    DeviceId,
    ElementState,
    Event,
    EventsLoop,
    EventsLoopClosed,
    EventsLoopProxy,
    Icon,
    KeyboardInput,
    ModifiersState,
    MonitorId,
    MouseButton,
    MouseCursor,
    MouseScrollDelta,
    ScanCode,
    Touch,
    TouchPhase,
    VirtualKeyCode,
    Window,
    WindowAttributes,
    WindowBuilder,
    WindowEvent,
    WindowId,
};
