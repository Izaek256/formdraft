//! The application shell: title bar, menu bar, navigation rail, pattern tools,
//! toolbar, inspector tabs, dialogs, startup splash, panel column and status
//! bar. Each region has a pure model and a thin view (D6).

pub mod controlbar;
pub mod dialogs;
pub mod menu;
pub mod notify;
pub mod panels;
pub mod rail;
pub mod shortcuts;
pub mod startup;
pub mod statusbar;
pub mod titlebar;
pub mod toolstrip;

pub use panels::PanelsModel;
pub use shortcuts::Action;
pub use toolstrip::{PatternTool, Tool};
