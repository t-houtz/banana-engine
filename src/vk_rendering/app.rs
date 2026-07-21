use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;
use winit::window::Window;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;

use super::app_data::AppData;
use super::instance::{VALIDATION_ENABLED, create_instance};
use super::device::pick_physical_device;

#[derive(Clone, Debug)]

pub struct App {
    entry: Entry,
    instance: Instance,
    data: AppData
}

impl App {
    // Create the Vulkan app
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;

        pick_physical_device(&instance, &mut data)?;
        
        Ok(Self { entry, instance, data })
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

        self.instance.destroy_instance(None);
    }
}