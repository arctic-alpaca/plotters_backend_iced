//! A [`glow`] renderer for [`iced_native`].
//!
//! [`glow`]: https://github.com/grovesNL/glow
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod backend;
mod program;
mod quad;
mod text;
mod triangle;

pub mod settings;
pub mod widget;
pub mod window;

pub use backend::Backend;
pub use settings::Settings;

pub(crate) use iced_graphics::Transformation;

#[doc(no_inline)]
pub use widget::*;

pub use iced_graphics::{Error, Viewport};
pub use iced_native::{
    Background, Color, Command, HorizontalAlignment, Length, Vector,
    VerticalAlignment,
};

/// A [`glow`] graphics renderer for [`iced`].
///
/// [`glow`]: https://github.com/grovesNL/glow
/// [`iced`]: https://github.com/hecrj/iced
pub type Renderer = iced_graphics::Renderer<Backend>;
