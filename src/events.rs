pub fn setup_ctrl_c() {
    #[cfg(debug_assertions)]
    {
        if ctrlc::set_handler(|| {
            crate::debug_log!("Ctrl+C recibido. Saliendo...");
            std::process::exit(0);
        })
        .is_ok()
        {
            crate::debug_log!("Modo debug: presiona Ctrl+C para salir.");
        }
    }
}
