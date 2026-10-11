//! The application shell: menu bar, tool strip, control bar, panel column and
//! status bar. Each region has a pure model and a thin view (D6).

pub mod controlbar;
pub mod menu;
pub mod panels;
pub mod shortcuts;
pub mod statusbar;
pub mod toolstrip;

pub use panels::PanelsModel;
pub use shortcuts::Action;
pub use toolstrip::Tool;
