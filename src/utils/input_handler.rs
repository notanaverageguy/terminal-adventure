use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// A structure for handling keyboard input asynchronously.
///
/// The `InputHandler` struct provides functionality for capturing and managing keyboard input
/// in a multithreaded environment. It tracks the state of each key (pressed or not) in a thread-safe
/// manner and operates in a separate thread to continuously listen for and process keyboard events.
///
/// ## Fields
///
/// * `key_states`: A shared, thread-safe map that maintains the state of each key. Each key is associated
///   with a boolean indicating whether it is pressed (`true`) or not (`false`).
/// * `running`: A shared atomic boolean that indicates whether the input handling thread should
///   continue running.
pub struct InputHandler {
    /// A shared, thread-safe map of key states where each key is associated with a boolean
    /// indicating whether the key is pressed (`true`) or not (`false`).
    pub key_states: Arc<Mutex<HashMap<KeyCode, bool>>>,

    pub input_mode: Arc<InputMode>,

    /// A shared atomic boolean that indicates whether the input handling thread should keep running.
    running: Arc<AtomicBool>,
}

impl InputHandler {
    /// Creates a new instance of `InputHandler` with initialized states.
    ///
    /// This constructor initializes a new `InputHandler` instance with an empty `HashMap` for key states
    /// and sets the `running` flag to `true`, indicating that the input handling thread should start running.
    ///
    /// ## Returns
    ///
    /// Returns an `InputHandler` instance with a new `HashMap` for key states and the running flag set to `true`.
    ///
    /// ## Example
    ///
    /// ```
    /// let input_handler = InputHandler::new();
    /// ```
    pub fn new() -> Self {
        let input_mode = if check_compatibility_mode() {
            InputMode::Compatibility
        } else {
            InputMode::Normal
        };

        Self {
            key_states: Arc::new(Mutex::new(HashMap::new())),
            running: Arc::new(AtomicBool::new(true)),

            input_mode: Arc::new(input_mode),
        }
    }

    /// Starts a new thread to handle keyboard input.
    ///
    /// This method spawns a new thread that continuously listens for keyboard events and updates the
    /// `key_states` map accordingly. The thread will keep running until `self.running` is set to `false`
    /// via the `stop` method.
    ///
    /// ## Example
    ///
    /// ```
    /// let input_handler = InputHandler::new();
    /// input_handler.start();
    /// ```
    pub fn start(&self) {
        let key_states_input = Arc::clone(&self.key_states);
        let running_input = Arc::clone(&self.running);
        let input_mode_threaded = Arc::clone(&self.input_mode);

        thread::spawn(move || {
            while running_input.load(Ordering::Relaxed) {
                if let Event::Key(key_event) = event::read().unwrap() {
                    let mut key_states = key_states_input.lock().unwrap();

                    match key_event.kind {
                        KeyEventKind::Press => {
                            // Mark the key as pressed
                            key_states.insert(key_event.code, true);
                        }
                        KeyEventKind::Release => {
                            if let InputMode::Normal = *input_mode_threaded {
                                // Remove the key from the map to indicate it's not pressed
                                key_states.remove(&key_event.code);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    /// Signals the input handling thread to stop running.
    ///
    /// This method sets the `running` flag to `false`, which causes the input handling thread to exit
    /// its loop and terminate. It should be called when you want to stop capturing keyboard input.
    ///
    /// ## Example
    ///
    /// ```
    /// let input_handler = InputHandler::new();
    /// input_handler.start();
    /// // After some time or condition
    /// input_handler.stop();
    /// ```
    pub fn stop(&self) {
        // Set `running` to false to stop the thread.
        self.running.store(false, Ordering::Relaxed);
    }

    /// Retrieves the current key states.
    ///
    /// This method locks the `key_states` map and returns a guard that provides access to the current
    /// key states. The key states map contains the state of each key (pressed or not).
    ///
    /// ## Returns
    ///
    /// Returns a `std::sync::MutexGuard` that provides access to the `HashMap` of key states.
    ///
    /// ## Example
    ///
    /// ```
    /// let input_handler = InputHandler::new();
    /// input_handler.start();
    /// // Later, get the key states
    /// let key_states = input_handler.get_key_states();
    /// ```
    pub fn get_key_states(&self) -> HashMap<KeyCode, bool> {
        let key_states: HashMap<KeyCode, bool> = self.key_states.lock().unwrap().clone();

        if let InputMode::Compatibility = *self.input_mode {
            self.key_states.lock().unwrap().clear();
        }

        key_states
    }

    /// Checks if a key was pressed and then marks that we checked if it was pressed already.
    ///
    /// This method retrieves the state of the specified `keycode` and, if the key is has not already been presse,
    /// sets its state to `false` (pressed) and returns `true`. If the key has already been pressed, it returns `false`.
    ///
    /// ## Arguments
    ///
    /// * `keycode`: The `KeyCode` of the key to check.
    ///
    /// ## Returns
    ///
    /// Returns `true` if the key was pressed and has been marked as released; otherwise, returns `false`.
    ///
    /// ## Example
    ///
    /// ```
    /// let input_handler = InputHandler::new();
    /// input_handler.start();
    /// // Later, check if a specific key was pressed
    /// if input_handler.get_key_once(&crossterm::event::KeyCode::Esc) {
    ///     println!("Escape key was pressed.");
    /// }
    /// ```
    pub fn get_key_once(&self, keycode: &event::KeyCode) -> bool {
        let mut key_states = self.key_states.lock().unwrap().clone();

        if key_states.get(keycode).unwrap_or(&false) == &true {
            key_states.insert(*keycode, false);
            return true;
        }

        false
    }
}

use std::env;

/// An enum representing the input mode.
///
/// This enum defines the modes in which the application can operate:
///
/// - `Normal`: Standard input mode, used for most systems.
/// - `Compatibility`: Compatibility mode for systems where certain inputs might not work as expected.
#[derive(Clone, Copy, Debug)]
pub enum InputMode {
    Normal,
    Compatibility,
}

/// Checks if the program was started with the `--compatibility-input` argument.
///
/// This function inspects the command-line arguments to determine if compatibility mode should be enabled.
///
/// ## Returns
/// - `true` if `--compatibility-input` is passed, indicating that compatibility mode should be used.
/// - `false` otherwise, meaning the program will use the normal input mode.
///
/// ## Example
/// ```rust
/// if check_compatibility_mode() {
///     println!("Compatibility mode enabled.");
/// } else {
///     println!("Normal mode.");
/// }
/// ```
fn check_compatibility_mode() -> bool {
    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg == "--compatibility-input" {
            return true;
        }
    }

    false
}
