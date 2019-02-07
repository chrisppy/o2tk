// Copyright © 2018-2019 red-oxide developers
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
pub mod prelude;

mod core;
mod oml;

use self::core::widgets::window::{
    Event,
    Window,
};
pub use self::{
    core::widgets::{
        window,
        Container,
        ContainerBuilder,
        Dock,
        DockBuilder,
        Label,
        LabelBuilder,
        Toolbar,
        ToolbarBuilder,
        WindowContainer,
        WindowContainerBuilder,
    },
    prelude::*,
};
use indexmap::IndexMap;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    sync::Arc,
};
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
    device::{
        Device,
        DeviceExtensions,
    },
    framebuffer::{
        Framebuffer,
        FramebufferAbstract,
        RenderPassAbstract,
        Subpass,
    },
    image::SwapchainImage,
    impl_vertex,
    instance::{
        Instance,
        PhysicalDevice,
    },
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
        self,
        FlushError,
        GpuFuture,
    },
};
use vulkano_win::{
    self,
    VkSurfaceBuild,
};
use winit::EventsLoop;

/// Trait to allow building the UI
pub trait UiBuild {
    /// Initialize the application
    fn init(app_id: &str) -> Result<Ui, Error>;
    /// Initialize themed application
    fn init_with_theme(app_id: &str, path: &str) -> Result<Ui, Error>;
    /// Add a widget to the application
    fn add_widget<'a>(&'a mut self, widget: Box<WidgetTrait>) -> &'a mut Ui;
    /// run the application
    fn run<F>(&self, wcontainer: Id, callback: F) -> Result<(), Error>
    where
        F: FnMut(Event, &Window) -> Run;
}

impl UiBuild for Ui {
    fn init(app_id: &str) -> Result<Self, Error> {
        let app_id = String::from(app_id);
        let theme = Theme::default();

        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        let widgets = IndexMap::new();
        let heirarchy = HashMap::new();

        Ok(Ui::new(app_id, theme, instance, heirarchy, widgets))
    }

    fn init_with_theme(app_id: &str, path: &str) -> Result<Self, Error> {
        let app_id = String::from(app_id);
        let theme = Theme::build(path)?;

        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        let widgets = IndexMap::new();
        let heirarchy = HashMap::new();

        Ok(Ui::new(app_id, theme, instance, heirarchy, widgets))
    }

    fn add_widget<'a>(&'a mut self, widget: Box<WidgetTrait>) -> &'a mut Self {
        let id = widget.id();
        let parent_id = widget.parent_id();

        self.widgets_mut().insert(id.clone(), Arc::new(Mutex::new(widget)));

        if let Some(pid) = parent_id {
            match self.heirarchy_mut().get_mut(&pid) {
                None => {
                    self.heirarchy_mut().insert(pid, vec![id.clone()]);
                }
                Some(v) => {
                    v.push(id.clone());
                }
            }
        }

        self
    }

    #[allow(clippy::ref_in_deref)]
    fn run<F>(&self, wcontainer_id: Id, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Event, &Window) -> Run,
    {
        let instance = self.clone().instance();

        let physical = match PhysicalDevice::enumerate(&instance).next() {
            Some(val) => val,
            None => {
                return Err(err_msg("no device available"));
            }
        };
        println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

        let mut events_loop = EventsLoop::new();
        let wcontainer = match self.widgets().get(&wcontainer_id) {
            None => {
                return Err(err_msg(format!(
                    "Could not find the window container: {}",
                    wcontainer_id
                )));
            }
            Some(val) => {
                let widget = val.lock();
                match widget.clone().downcast::<WindowContainer>() {
                    Err(_) => {
                        return Err(err_msg("Could not downcast to the window container"));
                    }
                    Ok(val) => val,
                }
            }
        };
        let surface = match wcontainer
            .window()
            .build_vk_surface(&events_loop, self.clone().instance())
        {
            Err(e) => {
                return Err(err_msg(e));
            }
            Ok(w) => w,
        };
        let window = surface.window();

        let queue_family = match physical
            .queue_families()
            .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        {
            Some(val) => val,
            None => {
                return Err(err_msg("couldn't find a graphical queue family"));
            }
        };

        let (device, mut queues) = {
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
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

        let dimensions = if let Some(dimensions) = window.get_inner_size() {
            let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        } else {
            return Ok(());
        };

        let (mut swapchain, images) = {
            let caps = match surface.capabilities(physical) {
                Err(err) => {
                    return Err(err_msg(format!("failed to get surface capabilities: {}", err)));
                }
                Ok(val) => val,
            };
            let usage = caps.supported_usage_flags;
            let alpha = match caps.supported_composite_alpha.iter().next() {
                None => return Err(err_msg("Next Composite alpha does not exist")),
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
                usage,
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

        // let mut draw_text = DrawText::new(device.clone(), queue.clone(), swapchain.clone(), &images);

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

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Box::new(sync::now(device.clone())) as Box<GpuFuture>;
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports:  None,
            scissors:   None,
        };
        let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        impl_vertex!(Vertex, position, color);
        let mut vertices = build_vertices(&self, dimensions)?;

        loop {
            previous_frame_end.cleanup_finished();

            if recreate_swapchain {
                let dimensions = if let Some(dimensions) = window.get_inner_size() {
                    let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
                    [dimensions.0, dimensions.1]
                } else {
                    return Ok(());
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

                swapchain = new_swapchain;
                framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);
                let a = build_vertices(&self, dimensions)?;
                vertices = a;
                recreate_swapchain = false;
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

            let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into()];

            let vertex_buffer =
                CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), vertices.clone().into_iter())
                    .unwrap();
            let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap()
                .begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
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
                    previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
                }
                Err(err) => {
                    println!("{:?}", err);
                    previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
                }
            }

            let mut done = false;
            events_loop.poll_events(|event| match callback(event, surface.window()) {
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

/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin:      [0.0, 0.0],
        dimensions:  [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vs.glsl"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/fs.glsl"
    }
}

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 2],
    color:    [f32; 4],
}

fn build_vertices(ui: &Ui, window_size: [u32; 2]) -> Result<Vec<Vertex>, Error> {
    let ui = normalize_sizes(ui)?;
    let h = window_size[1] as f32;
    let w = window_size[0] as f32;

    let mut vertices = Vec::new();
    for widget in ui.widgets().values() {
        let widget = widget.lock();
        let v = widget.draw(&ui)?;
        let mut vx1 = None;
        let mut vx2 = None;
        for (i, vertex) in v.iter().enumerate() {
            let pos = vertex.position();
            let vert = Vertex {
                position: pos.as_array(),
                color:    vertex.color(),
            };
            vertices.push(vert.clone());
            if i == 1 {
                vx1 = Some(pos);
            } else if i == 4 {
                vx2 = Some(pos);
            }
        }
        match &widget.widget_type() {
            WidgetType::Label => {
                if let Some(label) = widget.downcast_ref::<Label>() {
                    if label.visible() {
                        let text_size = label.text_size();
                        let vx1 = vx1.unwrap();
                        let vx2 = vx2.unwrap();
                        let _x = ((vx1.x()) * w) / 2.0;
                        let y1 = ((vx1.y()) * h) / 2.0;
                        let y2 = ((vx2.y()) * h) / 2.0;
                        let _size = (y1 - y2) * (text_size / 100.0);
                    }
                }
            }
            WidgetType::Button => unimplemented!(),
            _ => (),
        }
    }

    Ok(vertices)
}

fn normalize_sizes(ui: &Ui) -> Result<Ui, Error> {
    let ui = ui.clone();
    for children in ui.heirarchy().values() {
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
            let widgets = &ui.widgets();
            let widget = widgets[child].lock();
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
            match ui.widgets().get(child) {
                None => {
                    return Err(err_msg("NCError: Attempted to use an id that does not exist"));
                }
                Some(widget) => {
                    let mut widget = widget.lock();
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
    Ok(ui)
}
