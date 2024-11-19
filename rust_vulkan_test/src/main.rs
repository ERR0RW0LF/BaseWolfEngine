extern crate ash;
extern crate winit;
extern crate env_logger;
extern crate log;


use ash::vk;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    env_logger::init();
    let entry = unsafe {
        ash::Entry::new().unwrap()
    };
    let app_name = std::ffi::CString::new("BaseWolfEngine").unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("BaseWolfEngine")
        .build(&event_loop)
        .unwrap();

    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        application_version: 0,
        p_engine_name: app_name.as_ptr(),
        engine_version: 0,
        api_version: vk::make_version(1, 0, 0),
    };

    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::INSTANCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        pp_enabled_layer_names: std::ptr::null(),
        enabled_layer_count: 0,
        pp_enabled_extension_names: std::ptr::null(),
        enabled_extension_count: 0,
    };

    let instance: ash::Instance = unsafe {
        entry.create_instance(&create_info, None)
            .expect("Instance creation error")
    };

    // Main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
    });

    unsafe {
        instance.destroy_instance(None);
    }
}