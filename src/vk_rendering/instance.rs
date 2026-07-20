pub struct Instance {}

impl Instance {

    unsafe fn create_instance(window: &Window, entry:&Entry) -> Result<Instance> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"Banana Engine\0")
            .application_version(vk::make_version(1, 0, 0))
            .engine_name(b"... engine\0")
            .engine_version(vk::make_version(1, 0, 0))
            .api_version(vk::make_version(1, 0, 0));

        let mut extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let flags = if
            cfg!(target_os = "macos") &&
            entry.version()? >= PORTABILITY_MACOS_VERSION
        {
            info!("Enabling extensions for macOS portability.");
            extensions.push(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());
            extensions.push(vk::KHR_PORTABILITY_ENUMERATION_EXTENSION.name.as_ptr());
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };

        let info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .flags(flags);

        Ok(entry.create_instance(&info, None)?)
    }
}