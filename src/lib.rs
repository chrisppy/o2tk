// Copyright (C) 2018 red-oxide developers
// This program is free software: you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation, version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <http://www.gnu.org/licenses/>.

//! The oxygen toolkit (or simply o2tk) is a modern cross-platform, easy to use, GUI toolkit to
//! breathe life into your applications

#![deny(missing_docs)]

#[macro_use]
extern crate vulkano_shader_derive;

pub mod enums;
pub mod prelude;
pub mod traits;
pub mod utils;
pub mod widgets;

use self::{
    prelude::*,
    widgets::{
        window::{
            Event,
            EventsLoop,
            Window,
        },
        WindowContainer,
    },
};
pub use failure::{
    err_msg,
    Error,
};
use std::{
    collections::HashMap,
    mem,
    sync::Arc,
};
use uuid::Uuid;
use vulkano::{
    self,
    buffer::{
        BufferUsage,
        CpuAccessibleBuffer,
    },
    command_buffer::{
        AutoCommandBufferBuilder,
        DynamicState,
    },
    device::Device,
    framebuffer::{
        Framebuffer,
        Subpass,
    },
    impl_vertex,
    instance::{
        Instance,
        PhysicalDevice,
    },
    ordered_passes_renderpass,
    pipeline::{
        viewport::Viewport,
        GraphicsPipeline,
    },
    single_pass_renderpass,
    swapchain::{
        self,
        AcquireError,
        PresentMode,
        SurfaceTransform,
        Swapchain,
        SwapchainCreationError,
    },
    sync::{
        now,
        FlushError,
        GpuFuture,
    },
};
use vulkano_win::VkSurfaceBuild;

/// The position and color of the vertices to be drawn
#[derive(Debug, Clone, Copy)]
pub struct DrawVertex {
    position: [f32; 2],
    color:    [f32; 4],
}

impl DrawVertex {
    /// Initialize a vertex
    pub fn new(position: [f32; 2], color: [f32; 4]) -> Self {
        Self { position, color }
    }

    /// Retrieve the position of the vertex
    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Retrieve the color of the vertex
    pub fn color(&self) -> [f32; 4] {
        self.color
    }
}

/// The main UI structure
#[derive(Clone)]
pub struct Ui {
    theme:     Theme,
    instance:  Arc<Instance>,
    ids:       Vec<Uuid>,
    heirarchy: HashMap<Uuid, Vec<Uuid>>,
    widgets:   HashMap<Uuid, Box<Widget>>,
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui {
    /// Initialize the application
    pub fn new() -> Self {
        let theme = Theme::default();

        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        let widgets = HashMap::new();
        let heirarchy = HashMap::new();
        let ids = Vec::new();

        Self {
            instance,
            theme,
            widgets,
            heirarchy,
            ids,
        }
    }

    /// Generate a new ID
    fn gen_id(&self) -> Uuid {
        Uuid::new_v4()
    }

    /// Retrieve all the widgets
    fn widgets(&self) -> &HashMap<Uuid, Box<Widget>> {
        &self.widgets
    }

    /// Set the theme api
    pub fn with_theme(&mut self, path: &str) -> Result<&mut Self, Error> {
        self.theme = Theme::build(path)?;
        Ok(self)
    }

    /// Retrieve the themes set by the api
    pub fn theme(&self) -> Theme {
        self.clone().theme
    }

    fn normalize_sizes(&mut self) -> Result<(), Error> {
        for children in self.heirarchy.values() {
            let mut top_count: u16 = 0;
            let mut hcenter_count: u16 = 0;
            let mut bottom_count: u16 = 0;
            let mut left_count: u16 = 0;
            let mut vcenter_count: u16 = 0;
            let mut right_count: u16 = 0;

            let mut top_width_percent = 0.0;
            let mut center_width_percent = 0.0;
            let mut bottom_width_percent = 0.0;
            let mut left_height_percent = 0.0;
            let mut center_height_percent = 0.0;
            let mut right_height_percent = 0.0;

            for child in children {
                let widget = &self.widgets[&child];
                match widget.size() {
                    Size::Full => match widget.position() {
                        Position::TopLeft => {
                            top_count += 1;
                            left_count += 1;
                        }
                        Position::Top => {
                            top_count += 1;
                            vcenter_count += 1;
                        }
                        Position::TopRight => {
                            top_count += 1;
                            right_count += 1;
                        }
                        Position::Left => {
                            hcenter_count += 1;
                            left_count += 1;
                        }
                        Position::Center => {
                            hcenter_count += 1;
                            vcenter_count += 1;
                        }
                        Position::Right => {
                            hcenter_count += 1;
                            right_count += 1;
                        }
                        Position::BottomLeft => {
                            bottom_count += 1;
                            left_count += 1;
                        }
                        Position::Bottom => {
                            bottom_count += 1;
                            vcenter_count += 1;
                        }
                        Position::BottomRight => {
                            bottom_count += 1;
                            right_count += 1;
                        }
                    },
                    Size::Size(width, height) => match widget.position() {
                        Position::TopLeft => {
                            top_width_percent += width;
                            left_height_percent += height;
                        }
                        Position::Top => {
                            top_width_percent += width;
                            center_height_percent += height;
                        }
                        Position::TopRight => {
                            top_width_percent += width;
                            right_height_percent += height;
                        }
                        Position::Left => {
                            center_width_percent += width;
                            left_height_percent += height;
                        }
                        Position::Center => {
                            center_width_percent += width;
                            center_height_percent += height;
                        }
                        Position::Right => {
                            center_width_percent += width;
                            right_height_percent += height;
                        }
                        Position::BottomLeft => {
                            bottom_width_percent += width;
                            left_height_percent += height;
                        }
                        Position::Bottom => {
                            bottom_width_percent += width;
                            center_height_percent += height;
                        }
                        Position::BottomRight => {
                            bottom_width_percent += width;
                            right_height_percent += height;
                        }
                    },
                }
            }

            if top_width_percent > 100.0 {
                return Err(err_msg("The top width exceeds the maximum 100%"));
            } else if center_width_percent > 100.0 {
                return Err(err_msg("The center width exceeds the maximum 100%"));
            } else if bottom_width_percent > 100.0 {
                return Err(err_msg("The bottome width exceeds the maximum 100%"));
            } else if left_height_percent > 100.0 {
                return Err(err_msg("The left height exceeds the maximum 100%"));
            } else if center_height_percent > 100.0 {
                return Err(err_msg("The center height exceeds the maximum 100%"));
            } else if right_height_percent > 100.0 {
                return Err(err_msg("The right height exceeds the maximum 100%"));
            }
            for child in children {
                match self.widgets.get_mut(child) {
                    None => {
                        return Err(err_msg("NCError: Attempted to use an id that does not exist"));
                    }
                    Some(widget) => {
                        if let Size::Full = widget.size() {
                            let size = match widget.position() {
                                Position::TopLeft => {
                                    let full_width = (100.0 - top_width_percent) / f32::from(top_count);
                                    let full_height = (100.0 - left_height_percent) / f32::from(left_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::Top => {
                                    let full_width = (100.0 - top_width_percent) / f32::from(top_count);
                                    let full_height = (100.0 - center_height_percent) / f32::from(vcenter_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::TopRight => {
                                    let full_width = (100.0 - top_width_percent) / f32::from(top_count);
                                    let full_height = (100.0 - right_height_percent) / f32::from(right_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::Left => {
                                    let full_width = (100.0 - center_width_percent) / f32::from(hcenter_count);
                                    let full_height = (100.0 - left_height_percent) / f32::from(left_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::Center => {
                                    let full_width = (100.0 - center_width_percent) / f32::from(hcenter_count);
                                    let full_height = (100.0 - center_height_percent) / f32::from(vcenter_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::Right => {
                                    let full_width = (100.0 - center_width_percent) / f32::from(hcenter_count);
                                    let full_height = (100.0 - right_height_percent) / f32::from(right_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::BottomLeft => {
                                    let full_width = (100.0 - bottom_width_percent) / f32::from(bottom_count);
                                    let full_height = (100.0 - left_height_percent) / f32::from(left_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::Bottom => {
                                    let full_width = (100.0 - bottom_width_percent) / f32::from(bottom_count);
                                    let full_height = (100.0 - center_height_percent) / f32::from(vcenter_count);
                                    Size::Size(full_width, full_height)
                                }
                                Position::BottomRight => {
                                    let full_width = (100.0 - bottom_width_percent) / f32::from(bottom_count);
                                    let full_height = (100.0 - right_height_percent) / f32::from(right_count);
                                    Size::Size(full_width, full_height)
                                }
                            };
                            widget.set_size(size);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Add a widget to the application
    pub fn add_widget(self, widget: Box<Widget>) -> Self {
        let id = widget.id();
        let parent_id = widget.parent_id();

        let mut w = self.widgets.clone();
        w.insert(id, widget);

        let mut h = self.heirarchy.clone();
        if let Some(pid) = parent_id {
            match h.get_mut(&pid) {
                None => {
                    h.insert(pid, vec![id]);
                }
                Some(v) => {
                    v.push(id);
                }
            }
        }
        let mut ids = self.ids.clone();
        ids.push(id);

        Self {
            theme: self.theme,
            instance: self.instance,
            widgets: w,
            heirarchy: h,
            ids,
        }
    }

    /// run the application
    #[allow(clippy::ref_in_deref)]
    pub fn run<F>(&mut self, wcontainer: &WindowContainer, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Event, &Window) -> Run,
    {
        self.normalize_sizes()?;
        let window = wcontainer.clone().window();
        let mut events = EventsLoop::new();

        let surface = match window.build_vk_surface(&events, self.clone().instance) {
            Err(e) => {
                return Err(err_msg(e));
            }
            Ok(w) => w,
        };

        let instance = self.clone().instance;
        let physical = match PhysicalDevice::enumerate(&instance).next() {
            Some(val) => val,
            None => {
                return Err(err_msg("no device available"));
            }
        };

        let mut dimensions = {
            let size = match surface.window().get_inner_size() {
                None => {
                    return Err(err_msg("Could not retrieve the window's inner size"));
                }
                Some(val) => val,
            };
            [size.width as u32, size.height as u32]
        };

        let queue_family = match physical.queue_families().find(|&q| q.supports_graphics()) {
            Some(val) => val,
            None => {
                return Err(err_msg("couldn't find a graphical queue family"));
            }
        };

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                ..vulkano::device::DeviceExtensions::none()
            };

            match Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue_family, 0.5)].iter().cloned(),
            ) {
                Err(err) => {
                    return Err(err_msg(format!("failed to create device: {}", err)));
                }
                Ok(val) => val,
            }
        };

        let queue = match queues.next() {
            None => {
                return Err(err_msg("Could not retrieve the next queue"));
            }
            Some(val) => val,
        };

        let (mut swapchain, mut images) = {
            let caps = match surface.capabilities(physical) {
                Err(err) => {
                    return Err(err_msg(format!("failed to get surface capabilities: {}", err)));
                }
                Ok(val) => val,
            };

            let alpha = match caps.supported_composite_alpha.iter().next() {
                None => return Err(err_msg("Next Composite alpha does not exist")),
                Some(val) => val,
            };

            dimensions = match caps.current_extent {
                None => dimensions,
                Some(val) => val,
            };

            let format = caps.supported_formats[0].0;

            match Swapchain::new(
                device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                caps.supported_usage_flags,
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                true,
                None,
            ) {
                Err(err) => {
                    return Err(err_msg(format!("failed to create swapchain: {}", err)));
                }
                Ok(val) => val,
            }
        };

        let vertex_buffer = {
            #[derive(Debug, Clone)]
            struct Vertex {
                position: [f32; 2],
                color:    [f32; 4],
            }

            impl_vertex!(Vertex, position, color);

            let mut vertices: Vec<Vertex> = Vec::new();

            for id in &self.ids {
                let widget = &self.widgets[&id];
                for vertex in widget.draw(&self)? {
                    vertices.push(Vertex {
                        position: vertex.position(),
                        color:    vertex.color(),
                    });
                }
            }

            // TODO: change buffer
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), vertices.into_iter()).unwrap()
        };

        let vs = match vs::Shader::load(device.clone()) {
            Err(err) => {
                return Err(err_msg(format!("failed to create vertex shader module: {}", err)));
            }
            Ok(val) => val,
        };

        let fs = match fs::Shader::load(device.clone()) {
            Err(err) => {
                return Err(err_msg(format!("failed to create fragment shader module: {}", err)));
            }
            Ok(val) => val,
        };

        let single_pass = match single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }) {
            Err(err) => {
                return Err(err_msg(err));
            }
            Ok(val) => val,
        };

        let render_pass = Arc::new(single_pass);

        let subpass = match Subpass::from(render_pass.clone(), 0) {
            None => {
                return Err(err_msg("Could not retrieve subpass"));
            }
            Some(val) => val,
        };

        let gpipeline = match GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(subpass)
            .build(device.clone())
        {
            Err(err) => {
                return Err(err_msg(format!("failed to create graphics pipeline: {}", err)));
            }
            Ok(val) => val,
        };

        let pipeline = Arc::new(gpipeline);

        let mut framebuffers: Option<Vec<Arc<Framebuffer<_, _>>>> = None;

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;

        loop {
            previous_frame_end.cleanup_finished();

            if recreate_swapchain {
                dimensions = {
                    let size = match surface.window().get_inner_size() {
                        None => {
                            return Err(err_msg("Could not retrieve the window's inner size"));
                        }
                        Some(val) => val,
                    };
                    [size.width as u32, size.height as u32]
                };

                let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => {
                        continue;
                    }
                    Err(err) => {
                        return Err(err_msg(format!("{}", err)));
                    }
                };

                mem::replace(&mut swapchain, new_swapchain);
                mem::replace(&mut images, new_images);

                framebuffers = None;

                recreate_swapchain = false;
            }

            if framebuffers.is_none() {
                let new_framebuffers = Some(
                    images
                        .iter()
                        .map(|image| {
                            Arc::new(
                                Framebuffer::start(render_pass.clone())
                                    .add(image.clone())
                                    .unwrap()
                                    .build()
                                    .unwrap(),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
                mem::replace(&mut framebuffers, new_framebuffers);
            }

            let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    recreate_swapchain = true;
                    continue;
                }
                Err(err) => {
                    return Err(err_msg(format!("{}", err)));
                }
            };

            let dynamic_state = DynamicState {
                line_width: None,
                viewports:  Some(vec![Viewport {
                    origin:      [0.0, 0.0],
                    dimensions:  [dimensions[0] as f32, dimensions[1] as f32],
                    depth_range: 0.0..1.0,
                }]),
                scissors:   None,
            };

            let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap()
                .begin_render_pass(
                    framebuffers.as_ref().unwrap()[image_num].clone(),
                    false,
                    vec![[0.0, 0.0, 1.0, 1.0].into()],
                )
                .unwrap()
                .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ())
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap();

            let future = previous_frame_end
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Box::new(future) as Box<_>;
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Box::new(now(device.clone())) as Box<_>;
                }
                Err(err) => {
                    println!("{:?}", err);
                    previous_frame_end = Box::new(now(device.clone())) as Box<_>;
                }
            }

            let mut done = false;
            events.poll_events(|event| match callback(event, surface.window()) {
                Run::Continue => (),
                Run::Done => done = true,
                Run::Redraw => recreate_swapchain = true,
            });

            if done {
                return Ok(());
            }
        }
    }
}

mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[path = "src/utils/vs.glsl"]
    #[allow(dead_code)]
    struct Dummy;
}

mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[path = "src/utils/fs.glsl"]
    #[allow(dead_code)]
    struct Dummy;
}
