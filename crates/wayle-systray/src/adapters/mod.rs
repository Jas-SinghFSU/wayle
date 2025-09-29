/// UI framework adapters for the systray service.
///
/// These adapters enable integration with various GUI toolkits,
/// allowing systray menus to be displayed natively in different frameworks.

#[cfg(feature = "adapter-gtk")]
pub mod gtk;
