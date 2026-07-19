pub fn choose_physical_device(instance: VkInstance) -> VkPhysicalDevice {
    // ----------------------------
    // Enumerate Physical Devices
    // ----------------------------
    let mut phys_device_count: u32 = 0;
    let result = unsafe {
        vkEnumeratePhysicalDevices(
            instance,
            &mut phys_device_count,
            core::ptr::null_mut()
        )
    };

    if result != VK_SUCCESS {
        panic!("Failed to enumerate physical devices. Error: {:?}.", result);
    }
    if phys_device_count == 0 {
        panic!("No vulkan capable device found.");
    }
    let mut phys_devices = vec![core::ptr::null_mut(); phys_device_count as usize];
    let result = unsafe {
        vkEnumeratePhysicalDevices(
            instance,
            &mut phys_device_count,
            phys_devices.as_mut_ptr()
        )
    };

    if result != VK_SUCCESS {
        panic!("Failed to enumerate physical devices. Error: {:?}.", result);
    }
    let phys_device_index: u32 = 0;
    let chosen_phys_device = phys_devices[phys_device_index];



    // ----------------------------
    // Check Device Capabilities
    // ----------------------------
    
    let mut phys_device_properties = VkPhysicalDeviceProperties::default();
    unsafe {
        vkGetPhysicalDeviceProperties(
            chosen_phys_device,
            &mut phys_device_properties
        );
    }
    let device_name = unsafe {
        core::ffi::CStr::from_ptr(
            phys_device_properties.deviceName.as_ptr()
        )
    };
    println!("Chosen device name: {:?}", device_name);
}

pub fn find_graphics_queue_family(physical_device: VkPhysicalDevice) -> u32 {
    // ----------------------------
    // Checking Queues
    // ----------------------------
    let mut queue_family_count: u32 = 0;
    unsafe {
        vkGetPhysicalDeviceQueueFamilyProperties(
            chosen_phys_device,
            &mut queue_family_count,
            core::ptr::null_mut()
        );
    }

    let mut queue_families = vec![VkQueueFamilyProperties::default(); queue_family_count as usize];
    unsafe {
        vkGetPhysicalDeviceQueueFamilyProperties(
            chosen_phys_device,
            &mut queue_family_count,
            queue_families.as_mut_ptr()
        );
    }

    let mut chosen_graphics_queue_family: i32 = -1;
    let mut chosen_graphics_queue_index: u32 = 0;
    for i in 0..queue_families.len()
    {
        let queue_family_index = i as i32;
        let queue_family = &queue_families[i];
        if queue_family.queueFlags & VK_QUEUE_GRAPHICS_BIT as VkQueueFlags != 0 {
            chosen_graphics_queue_family = queue_family_index;
            chosen_graphics_queue_index = 0;
        }
    }

    if chosen_graphics_queue_family == -1 {
        panic!("Chosen physical device is not suitable.");
    }
    let chosen_graphics_queue_family = chosen_graphics_queue_family as u32;
}