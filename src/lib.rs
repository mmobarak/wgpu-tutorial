use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

// Define a struct to hold application state (like the window)
struct App {
    window: Option<Arc<Window>>, // Use Arc for shared ownership if needed later
}

impl ApplicationHandler for App {
    // --- Application Lifecycle ---

    // Called once the event loop is running and ready
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the window only if it doesn't exist
        if self.window.is_none() {
            println!("App Resumed: Creating window...");
            let attributes = WindowAttributes::default() // Use WindowAttributes
                .with_title("Minimal winit App (0.30)");
            // Create window using the active event loop
            let window = event_loop
                .create_window(attributes)
                .expect("Failed to create window");
            self.window = Some(Arc::new(window));
        }
    }

    // --- Window Events ---
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Ensure the event is for our window (if it exists)
        if self.window.as_ref().map_or(false, |w| w.id() == window_id) {
            match event {
                // Close requested (e.g., clicking the 'X' button)
                WindowEvent::CloseRequested => {
                    println!("Close requested, exiting...");
                    event_loop.exit(); // Use exit() on ActiveEventLoop
                }
                // Keyboard input
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: winit::event::ElementState::Pressed,
                            .. // Ignore other fields
                        },
                    .. // Ignore other fields
                } => {
                    println!("Escape key pressed, exiting...");
                    event_loop.exit(); // Use exit() on ActiveEventLoop
                }
                // Other window events (resize, moved, etc.) - we ignore them here
                _ => (),
            }
        }
    }

    // Optional: Called just before the application exits
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Exiting application.");
    }
}

// Public run function remains the entry point
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Create the event loop
    let event_loop = EventLoop::new()?;

    // Set control flow (optional, default is Wait)
    // Wait is generally preferred for GUI apps unless constant updates are needed
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App { window: None }; // Initialize state

    // Run the event loop
    // The run_app method takes the ApplicationHandler and drives the loop
    event_loop.run_app(&mut app)?;

    Ok(())
}
