/// Device management functionality.
pub mod device;
/// Stream management functionality.
pub mod stream;

pub use device::{InputDevice, OutputDevice};
pub use stream::AudioStream;
