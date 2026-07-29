use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;
use winit::window::Window;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::{ExtDebugUtilsExtensionInstanceCommands, KhrSurfaceExtensionInstanceCommands, KhrSwapchainExtensionDeviceCommands};
use vulkanalia::window as vk_window;

use super::app_data::AppData;
use super::instance::{VALIDATION_ENABLED, create_instance};
use super::device::{pick_physical_device, create_logical_device};
use super::swapchain::{create_swapchain, create_swapchain_image_views};
use super::pipeline::create_pipeline;

#[derive(Clone, Debug)]

pub struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device
}

impl App {
    // Create the Vulkan app
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;

        data.surface = vk_window::create_surface(&instance, &window, &window)?;

        pick_physical_device(&instance, &mut data)?;

        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        
        Ok(Self { entry, instance, data, device })
    }

    // Renders a frame
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    // Destroys the app
    pub unsafe fn destroy(&mut self) {
        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.data.swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }
}