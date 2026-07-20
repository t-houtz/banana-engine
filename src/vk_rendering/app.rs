#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance
}

impl App {
    // Create the Vulkan app
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    // Renders a frame
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    // Destroys the app
    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}