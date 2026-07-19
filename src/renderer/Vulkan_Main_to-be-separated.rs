use vk_bindings::*;

fn main()
{
    

    // ----------------------------
    // Cleanup
    // ----------------------------

    println!("Deleting device.");
    unsafe
    {
        vkDestroyDevice(
            device,
            core::ptr::null()
        );
    }


    println!("Deleting instance.");
    unsafe {
        vkDestroyInstance(
            instance,
            core::ptr::null()
        )
    }
}