mod renderer;

use renderer::device::create_device;
use renderer::instance::create_instance;
use renderer::physical_device::{
    choose_physical_device,
    find_graphics_queue_family,
};

fn main() {
    let instance = create_instance();

    let physical_device =
        choose_physical_device(instance);

    let graphics_queue_family =
        find_graphics_queue_family(physical_device);

    let (device, graphics_queue) =
        create_device(
            physical_device,
            graphics_queue_family,
        );

    unsafe {
        vkDestroyDevice(
            device,
            core::ptr::null(),
        );

        vkDestroyInstance(
            instance,
            core::ptr::null(),
        );
    }
}