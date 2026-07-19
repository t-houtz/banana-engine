pub fn create_device(
    physical_device: VkPhysicalDevice,
    graphics_queue_family: u32
) -> (VkDevice, VkQueue)   
    // ----------------------------
    // Device Creation
    // ----------------------------

    let queue_priority: f32 = 1.0;
    let queue_create_info = VkDeviceQueueCreateInfo {
        sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        pNext: core::ptr::null(),
        flags: 0x0,
        queueFamilyIndex: chosen_graphics_queue_family,
        queueCount: 1,
        pQueuePriorities: &queue_priority
    }
    let phys_device_features = VkPhysicalDeviceFeatures::default();
    let device_create_info = VkDeviceCreateInfo {
        sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: core::ptr::null(),
        flags: 0x0,
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &queue_create_info,
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: core::ptr::null(),
        pEnabledFeatures: &phys_device_features
    };

    println!("Creating device.")
    let mut device = core::ptr::null_mut();
    let result = unsafe {
        vkCreateDevice(
            chosen_phys_device,
            &device_create_info,
            core::ptr::null_mut(),
            &mut device
        )
    };
    if result != VK_SUCCESS {
        panic!("Failed to create vulkan device. Error: {:?}.", result);
    }

    // ----------------------------
    // ???
    // ---------------------------
    let mut graphics_queue = core::ptr::null_mut();
    unsafe
    {
        vkGetDeviceQueue(
            device,
            chosen_graphics_queue_family,
            chosen_graphics_queue_index,
            &mut graphics_queue
        );
    }
