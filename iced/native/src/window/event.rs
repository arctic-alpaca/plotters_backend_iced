use std::path::PathBuf;

/// A window-related event.
#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    /// A window was resized
    Resized {
        /// The new width of the window (in units)
        width: u32,

        /// The new height of the window (in units)
        height: u32,
    },

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted
    /// for each file separately.
    FileHovered(PathBuf),

    /// A file has beend dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted
    /// for each file separately.
    FileDropped(PathBuf),

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `FilesHoveredLeft` event triggered even if
    /// multiple files were hovered.
    FilesHoveredLeft,
}
