use super::super::{
    super::{
        context::Context,
        game_loop::GameLoop,
    }
};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId, WindowButtons, Icon},
    dpi::{Size, Position, LogicalSize, LogicalPosition}
};
use std::sync::Arc;

#[derive(Clone)]
pub struct WindowConfiguration{

}

impl Default for WindowConfiguration {
    fn default() -> Self {
        return Self {};
    }
}

struct Application {
    window: Option<Arc<Window>>,
    window_configuration: Option<WindowConfiguration>,
    context: Option<Context>,
    game_loop: GameLoop
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window: Arc<Window> = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        self.window = Some(window.clone());
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, window_event: WindowEvent) {}
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {}
}

pub async fn initialize_application(
    window_configuration: Option<WindowConfiguration>,
    setup: fn(context: &mut Context),
    update: fn(context: &mut Context)
) {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application: Application = if let Some(window_configuration_unwrapped) = window_configuration {
        Application {
            window: None,
            context: None,
            window_configuration: Some(window_configuration_unwrapped),
            game_loop: GameLoop::new(setup, update)
        }
    } else {
        Application {
            window: None,
            context: None,
            window_configuration: None,
            game_loop: GameLoop::new(setup, update)
        }
    };
    let _ = event_loop.run_app(&mut application);

    println!("App initted");
}