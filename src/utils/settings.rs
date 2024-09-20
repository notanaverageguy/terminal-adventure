use crossterm::event::KeyCode;

/// A struct representing all the settings for the player.
///
/// This struct contains key bindings for player movement and the input mode used for handling inputs.
///
/// ## Fields
/// - `move_up`: The key used to move the player up.
/// - `move_right`: The key used to move the player right.
/// - `move_left`: The key used to move the player left.
/// - `move_down`: The key used to move the player down.
/// - `input_mode`: The input mode, which determines how inputs are processed.
#[derive(Clone, Copy)]
pub struct Settings {
    pub move_up: KeyCode,
    pub move_right: KeyCode,
    pub move_left: KeyCode,
    pub move_down: KeyCode,
}

impl Settings {
    /// Creates a `Settings` instance with default key bindings and an appropriate input mode.
    ///
    /// The input mode is determined by checking if the `--compatibility-input` argument is passed.
    ///
    /// ## Returns
    /// A `Settings` struct initialized with default values:
    /// - `move_up` is bound to `KeyCode::Char('w')`.
    /// - `move_right` is bound to `KeyCode::Char('d')`.
    /// - `move_left` is bound to `KeyCode::Char('a')`.
    /// - `move_down` is bound to `KeyCode::Char('s')`.
    /// - `input_mode` is set based on the presence of the `--compatibility-input` argument.
    pub fn default() -> Self {
        Settings {
            move_up: KeyCode::Char('w'),
            move_right: KeyCode::Char('d'),
            move_left: KeyCode::Char('a'),
            move_down: KeyCode::Char('s'),
        }
    }
}
