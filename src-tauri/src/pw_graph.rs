


#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn inspect_registry() {
        let mainloop = pipewire::MainLoop::new().unwrap();
        let context = pipewire::Context::new(&mainloop).unwrap();
        let core = context.connect(None).unwrap();
        let registry = core.get_registry().unwrap();
        let _listener = registry
            .add_listener_local()
            .global(|global| println!("New global: {:?}", global))
            .register();
        mainloop.run()
    }
}
