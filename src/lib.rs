use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!("App Resumed: Creating window...");
            let attributes = WindowAttributes::default()
                .with_title("Minimal winit App (0.30)");
            let window = event_loop
                .create_window(attributes)
                .expect("Failed to create window");
            self.window = Some(Arc::new(window));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.window.as_ref().map_or(false, |w| w.id() == window_id) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Close requested, exiting...");
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    println!("Escape key pressed, exiting...");
                    event_loop.exit();
                }
                _ => (),
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Exiting application.");
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App { window: None };

    event_loop.run_app(&mut app)?;
    Ok(())
}
