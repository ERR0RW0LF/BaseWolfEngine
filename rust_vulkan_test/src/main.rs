extern crate vulkano;
extern crate vulkano_shaders;
extern crate winit;

use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::swapchain::{Surface, Swapchain};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::framebuffer::{Framebuffer, Subpass};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer};
use vulkano::sync::{GpuFuture, now};
use winit::{EventsLoop, WindowBuilder};

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), &[], None).unwrap();
    let events_loop = EventsLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    let physical = vulkano::instance::PhysicalDevice::enumerate(&instance).next().unwrap();
    let queue_family = physical.queue_families().find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false)).unwrap();

    let (device, mut queues) = Device::new(physical, &physical.supported_features(), &DeviceExtensions::none(), [(queue_family, 0.5)].iter().cloned()).unwrap();
    let queue = queues.next().unwrap();

    let (swapchain, images) = {
        let caps = surface.capabilities(physical).unwrap();
        let dimensions = caps.current_extent.unwrap_or([1024, 768]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format, dimensions, 1, caps.supported_usage_flags, &queue, vulkano::swapchain::SurfaceTransform::Identity, alpha, vulkano::swapchain::PresentMode::Fifo, true, None).unwrap()
    };

    // Main loop
    loop {
        events_loop.poll_events(|_| {});
    }
}