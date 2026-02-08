use std::str::FromStr;
use clap::Parser;

#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[repr(u8)]
pub enum Pattern {
    Percentage = 0,
    Gradient = 1,
    DoubleGradient = 2,
    LotusSideways = 3,
    Zigzag = 4,
    AllOn = 5,
    Panic = 6,
    LotusTopDown = 7,
    //AllBrightnesses
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[repr(u8)]
pub enum Game {
    Snake = 0,
    Pong = 1,
    Tetris = 2,
    GameOfLife = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[repr(u8)]
pub enum AddonAnimation {
    Spiral = 0x00,
    Splashes = 0x01,
}

#[derive(Copy, Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum GameOfLifeStartParam {
    CurrentMatrix = 0x00,
    Pattern1 = 0x01,
    Blinker = 0x02,
    Toad = 0x03,
    Beacon = 0x04,
    Glider = 0x05,
    BeaconToadBlinker = 0x06,
}

#[derive(Debug, Clone)]
pub struct KeypressArg {
    pub keycode: u16,
    pub pressed: bool,
}
impl FromStr for KeypressArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let keycode = parts.next().ok_or("missing key")?.parse().map_err(|_| "bad key")?;
        let pressed = parts.next().ok_or("missing bool")?.parse().map_err(|_| "bad bool")?;
        Ok(KeypressArg { keycode, pressed })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum Side {
    Left,
    Right,
}

/// LED Matrix
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct LedMatrixSubcommand {
    // addon stuff
    /// Sends keypresses to the matrix, by keycode.
    #[arg(long)]
    pub keypress: Option<KeypressArg>,
    /// Sets the current addon animation.
    #[arg(long)]
    #[clap(value_enum)]
    pub set_addon_animation: Option<AddonAnimation>,
    #[clap(long)]
    pub stop_addon_animation: bool,
    #[clap(long)]
    pub set_side: Option<Side>,

    /// Set LED max brightness percentage or get, if no value provided
    #[arg(long)]
    pub brightness: Option<Option<u8>>,

    /// Set sleep status or get, if no value provided
    #[arg(long)]
    pub sleeping: Option<Option<bool>>,

    /// Jump to the bootloader
    #[arg(long)]
    pub bootloader: bool,

    /// Display a percentage (0-100)
    #[arg(long)]
    pub percentage: Option<u8>,

    /// Start/stop animation
    #[arg(long)]
    pub animate: Option<Option<bool>>,

    /// Display a pattern
    #[arg(long)]
    #[clap(value_enum)]
    pub pattern: Option<Pattern>,

    /// Show every brightness, one per pixel
    #[arg(long)]
    pub all_brightnesses: bool,

    /// Blink the current pattern once a second
    #[arg(long)]
    pub blinking: bool,

    /// Breathing brightness of the current pattern
    #[arg(long)]
    pub breathing: bool,

    /// Display black&white image (9x34px)
    #[arg(long)]
    pub image_bw: Option<String>,

    /// Display grayscale image
    #[arg(long)]
    pub image_gray: Option<String>,

    /// Random EQ
    #[arg(long)]
    pub random_eq: bool,

    /// Display EQ of microphone input
    #[cfg(feature = "audio-visualizations")]
    #[arg(long)]
    pub input_eq: bool,

    /// EQ with custom values
    #[arg(long, num_args(9))]
    pub eq: Option<Vec<u8>>,

    /// Clock
    #[arg(long)]
    pub clock: bool,

    /// Display a string (max 5 chars)
    #[arg(long)]
    pub string: Option<String>,

    /// Display a string (max 5 symbols)
    #[arg(long, num_args(0..6))]
    pub symbols: Option<Vec<String>>,

    /// Start a game
    #[arg(long)]
    #[clap(value_enum)]
    pub start_game: Option<Game>,

    /// Paramater for starting the game. Required for some games
    #[arg(long)]
    #[clap(value_enum)]
    pub game_param: Option<GameOfLifeStartParam>,

    /// Stop the currently running game
    #[arg(long)]
    #[clap(value_enum)]
    pub stop_game: bool,

    /// Set/get animation FPS
    #[arg(long)]
    pub animation_fps: Option<Option<u16>>,

    /// Set/get PWM Frequency in Hz
    #[arg(long)]
    #[clap(value_enum)]
    pub pwm_freq: Option<Option<u16>>,

    /// Set debug mode or get current mode, if no value provided
    #[arg(long)]
    pub debug_mode: Option<Option<bool>>,

    /// Crash the firmware (TESTING ONLY!)
    #[arg(long)]
    pub panic: bool,

    /// Get the device version
    #[arg(short, long)]
    pub version: bool,
}
