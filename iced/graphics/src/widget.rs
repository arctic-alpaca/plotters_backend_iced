//! Use the widgets supported out-of-the-box.
//!
//! # Re-exports
//! For convenience, the contents of this module are available at the root
//! module. Therefore, you can directly type:
//!
//! ```
//! use iced_graphics::{button, Button};
//! ```
pub mod button;
pub mod checkbox;
pub mod container;
pub mod image;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod text_input;

mod column;
mod row;
mod space;
mod text;

#[doc(no_inline)]
pub use button::Button;
#[doc(no_inline)]
pub use checkbox::Checkbox;
#[doc(no_inline)]
pub use container::Container;
#[doc(no_inline)]
pub use pane_grid::PaneGrid;
#[doc(no_inline)]
pub use pick_list::PickList;
#[doc(no_inline)]
pub use progress_bar::ProgressBar;
#[doc(no_inline)]
pub use radio::Radio;
#[doc(no_inline)]
pub use rule::Rule;
#[doc(no_inline)]
pub use scrollable::Scrollable;
#[doc(no_inline)]
pub use slider::Slider;
#[doc(no_inline)]
pub use text_input::TextInput;

pub use column::Column;
pub use image::Image;
pub use row::Row;
pub use space::Space;
pub use svg::Svg;
pub use text::Text;

#[cfg(feature = "canvas")]
#[cfg_attr(docsrs, doc(cfg(feature = "canvas")))]
pub mod canvas;

#[cfg(feature = "canvas")]
#[doc(no_inline)]
pub use canvas::Canvas;
