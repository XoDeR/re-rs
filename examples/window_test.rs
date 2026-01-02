use re_rs::core::managers::windowing::manager;

fn main() {
    use re_rs::core::managers::windowing::manager::WindowConfiguration;
    use re_rs::core::context::Context;

    fn setup(context: &mut Context) {}
    fn update(context: &mut Context) {}
    
    pollster::block_on(re_rs::core::managers::windowing::manager::initialize_application(
        Some(WindowConfiguration::default()), setup, update
    ))
}
