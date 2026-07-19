    
pub fn create_instance() -> VkInstance {
    // ----------------------------
    // Layers
    // ----------------------------
    let std_validation_layer = b"VK_LAYER_KHRONOS_validation\0";
    let layers = [std_validation_layer.as_ptr() as *const i8];

    let mut available_layer_count = 0;
    let mut available_layers = Vec::new();
    unsafe
    {
        vkEnumerateInstanceLayerProperties(
            &mut available_layer_count,
            core::ptr::null_mut()
        );
    }

    available_layers.resize(available_layer_count as usize, VkLayerProperties::default());
    unsafe
    {
        vkEnumerateInstanceLayerProperties(
            &mut available_layer_count,
            available_layers.as_mut_ptr()
        );
    }

    for layer in layers.iter()
    {
        let layer = unsafe { core::ffi::CStr::from_ptr(*layer) };
        let mut found = false;
        for available_layer in available_layers.iter()
        {
            let available_layer = unsafe
            {
                core::ffi::CStr::from_ptr(
                    available_layer.layerName.as_ptr()
                )
            };

            if layer == available_layer
            {
                found = true;
            }
        }

        if !found
        {
            println!("Layer {:?} is not supported.", layer);
        }
    }



    // ----------------------------
    // Instance Creation
    // ----------------------------

    let app_name_byes : str = b"vk rust\0";
    let app_name = unsafe
    {
        core::ffi::Cstr::from_bytes_with_nul_uncheck(
            app_name_bytes
        )
    };

    let application_info = VkApplicationInfo {
        sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: core::ptr::null(),
        pApplicationName: app_name.as_ptr(),
        applicationVersion: make_version(0, 0, 1, 0),
        pEngineName: engine_name.as_ptr(),
        engineVersion: make_version(0, 0, 1, 0),
        apiVersion: make_version(0, 1, 0, 0)
    };

    let create_info = VkInstanceCreateInfo {
        sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: core::ptr::null(),
        flags: 0x0,
        pApplicationInfo: &application_info,
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: core::ptr::null(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: core::ptr::null()
    };

    println!("Creating Vulkan Instance");
    let mut instance = core::ptr::null_mut();
    let result = unsafe {
        vkCreateInstance(
            &create_infor,
            core::ptr::null(),
            &mut instance
        )
    };

    if result != VK_SUCCESS {
        panic!("Failed to create instance. Error: {:?}.", result);
    }
}