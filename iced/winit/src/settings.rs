//! Configure your application.
#[cfg(target_os = "windows")]
#[path = "settings/windows.rs"]
mod platform;
#[cfg(not(target_os = "windows"))]
#[path = "settings/not_windows.rs"]
mod platform;

pub use platform::PlatformSpecific;

use crate::conversion;
use crate::Mode;
use winit::monitor::MonitorHandle;
use winit::window::WindowBuilder;

/// The settings of an application.
#[derive(Debug, Clone, Default)]
pub struct Settings<Flags> {
    /// The [`Window`] settings
    ///
    /// [`Window`]: struct.Window.html
    pub window: Window,

    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: trait.Application.html
    pub flags: Flags,
}

/// The window settings of an application.
#[derive(Debug, Clone)]
pub struct Window {
    /// The size of the window.
    pub size: (u32, u32),

    /// The minimum size of the window.
    pub min_size: Option<(u32, u32)>,

    /// The maximum size of the window.
    pub max_size: Option<(u32, u32)>,

    /// Whether the window should be resizable or not.
    pub resizable: bool,

    /// Whether the window should have a border, a title bar, etc.
    pub decorations: bool,

    /// Whether the window should be transparent
    pub transparent: bool,

    /// The window icon, which is also usually used in the taskbar
    pub icon: Option<winit::window::Icon>,

    /// Platform specific settings.
    pub platform_specific: platform::PlatformSpecific,
}

impl Window {
    /// Converts the window settings into a `WindowBuilder` from `winit`.
    pub fn into_builder(
        self,
        title: &str,
        mode: Mode,
        primary_monitor: MonitorHandle,
    ) -> WindowBuilder {
        let mut window_builder = WindowBuilder::new();

        let (width, height) = self.size;

        window_builder = window_builder
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize { width, height })
            .with_resizable(self.resizable)
            .with_decorations(self.decorations)
            .with_transparent(self.transparent)
            .with_window_icon(self.icon)
            .with_fullscreen(conversion::fullscreen(primary_monitor, mode));

        if let Some((width, height)) = self.min_size {
            window_builder = window_builder
                .with_min_inner_size(winit::dpi::LogicalSize { width, height });
        }

        if let Some((width, height)) = self.max_size {
            window_builder = window_builder
                .with_max_inner_size(winit::dpi::LogicalSize { width, height });
        }

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowBuilderExtWindows;

            if let Some(parent) = self.platform_specific.parent {
                window_builder = window_builder.with_parent_window(parent);
            }
        }

        window_builder
    }
}

impl Default for Window {
    fn default() -> Window {
        Window {
            size: (1024, 768),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            platform_specific: Default::default(),
        }
    }
}
