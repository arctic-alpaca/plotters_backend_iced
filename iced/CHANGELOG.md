# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `"system_font"` feature gates reading system fonts. [#370]

[#370]: https://github.com/hecrj/iced/pull/370

## [0.1.1] - 2020-04-15
### Added
- `Settings::with_flags` to easily initialize some default settings with flags. [#266]
- `Default` implementation for `canvas::layer::Cache`. [#267]
- `Ctrl + Del` support for `TextInput`. [#268]
- Helper methods in `canvas::Path` to easily draw lines, rectangles, and circles. [#293]
- `From<Color>` implementation for `canvas::Fill`. [#293]
- `From<String>` implementation for `canvas::Text`. [#293]
- `From<&str>` implementation for `canvas::Text`. [#293]

### Changed
- `new` method of `Radio` and `Checkbox` now take a generic `Into<String>` for the label. [#260]
- `Frame::fill` now takes a generic `Into<canvas::Fill>`. [#293]
- `Frame::stroke` now takes a generic `Into<canvas::Stroke>`. [#293]
- `Frame::fill_text` now takes a generic `Into<canvas::Text>`. [#293]

### Fixed
- Feature flags not being referenced in documentation. [#259]
- Crash in some graphics drivers when displaying an empty `Canvas`. [#278]
- Text measuring when spaces where present at the beginning of a `TextInput` value. [#279]
- `TextInput` producing a `Clip` primitive when unnecessary. [#279]
- Alignment of `Text` primitive in `iced_wgpu`. [#281]
- `CursorEntered` and `CursorLeft` not being generated. [#289]

### Removed
- Unnecessary `'static` lifetimes in `Renderer` bounds. [#290]

[#259]: https://github.com/hecrj/iced/pull/259
[#260]: https://github.com/hecrj/iced/pull/260
[#266]: https://github.com/hecrj/iced/pull/266
[#267]: https://github.com/hecrj/iced/pull/267
[#268]: https://github.com/hecrj/iced/pull/268
[#278]: https://github.com/hecrj/iced/pull/278
[#279]: https://github.com/hecrj/iced/pull/279
[#281]: https://github.com/hecrj/iced/pull/281
[#289]: https://github.com/hecrj/iced/pull/289
[#290]: https://github.com/hecrj/iced/pull/290
[#293]: https://github.com/hecrj/iced/pull/293


## [0.1.0] - 2020-04-02
### Added
- __[Event subscriptions]__ (#122)  
  A declarative way to listen to external events asynchronously by leveraging [streams].

- __[Custom styling]__ (#146)  
  A simple, trait-based approach for customizing the appearance of different widgets.

- __[`Canvas` widget]__ (#193)  
  A widget for drawing 2D graphics with an interface inspired by the [Web Canvas API] and powered by [`lyon`].

- __[`PaneGrid` widget]__ (#224)  
  A widget that dynamically organizes layout by splitting panes that can be resized and drag and dropped.

- __[`Svg` widget]__ (#111)  
  A widget that renders vector graphics on top of [`resvg`] and [`raqote`]. Thanks to @Maldela!

- __[`ProgressBar` widget]__ (#141)  
  A widget to notify progress of asynchronous tasks to your users. Thanks to @Songtronix!

- __[Configurable futures executor]__ (#164)  
  Support for plugging [`tokio`], [`async-std`], [`wasm-bindgen-futures`], or your own custom futures executor to an application.

- __[Compatibility with existing `wgpu` projects]__ (#183)  
  A bunch of improvements to the flexibility of [`iced_wgpu`] to allow integration in existing codebases.

- __[Text selection for `TextInput`]__ (#202)  
  Thanks to @FabianLars and @Finnerale!

- __[Texture atlas for `iced_wgpu`]__ (#154)  
  An atlas on top of [`guillotiere`] for batching draw calls. Thanks to @Maldela!

[Event subscriptions]: https://github.com/hecrj/iced/pull/122
[Custom styling]: https://github.com/hecrj/iced/pull/146
[`Canvas` widget]: https://github.com/hecrj/iced/pull/193
[`PaneGrid` widget]: https://github.com/hecrj/iced/pull/224
[`Svg` widget]: https://github.com/hecrj/iced/pull/111
[`ProgressBar` widget]: https://github.com/hecrj/iced/pull/141
[Configurable futures executor]: https://github.com/hecrj/iced/pull/164
[Compatibility with existing `wgpu` projects]: https://github.com/hecrj/iced/pull/183
[Clipboard access]: https://github.com/hecrj/iced/pull/132
[Texture atlas for `iced_wgpu`]: https://github.com/hecrj/iced/pull/154
[Text selection for `TextInput`]: https://github.com/hecrj/iced/pull/202
[`lyon`]: https://github.com/nical/lyon
[`guillotiere`]: https://github.com/nical/guillotiere
[Web Canvas API]: https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API
[streams]: https://docs.rs/futures/0.3.4/futures/stream/index.html
[`tokio`]: https://github.com/tokio-rs/tokio
[`async-std`]: https://github.com/async-rs/async-std
[`wasm-bindgen-futures`]: https://github.com/rustwasm/wasm-bindgen/tree/master/crates/futures
[`resvg`]: https://github.com/RazrFalcon/resvg
[`raqote`]: https://github.com/jrmuizel/raqote
[`iced_wgpu`]: https://github.com/hecrj/iced/tree/0.1/wgpu


## [0.1.0-beta] - 2019-11-25
### Changed
- The old `iced` becomes `iced_native`. The current `iced` crate turns into a batteries-included, cross-platform GUI library.


## [0.1.0-alpha] - 2019-09-05
### Added
- First release! :tada:

[Unreleased]: https://github.com/hecrj/iced/compare/0.1.1...HEAD
[0.1.1]: https://github.com/hecrj/iced/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/hecrj/iced/compare/0.1.0-beta...0.1.0
[0.1.0-beta]: https://github.com/hecrj/iced/compare/0.1.0-alpha...0.1.0-beta
[0.1.0-alpha]: https://github.com/hecrj/iced/releases/tag/0.1.0-alpha
