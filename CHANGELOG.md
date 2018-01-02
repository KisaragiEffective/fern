0.5.1 (2017-12-26)
==================

- Re-add support for colored log levels with the 'colored' feature
  - This was initially implemented in fern 0.4.4 by @nihilus, but
    support was accidentally dropped in fern 0.5.0.
- Fix the ability to run tests on windows, and refactor integration
  tests for developer clarity
- Update documentation for clarity

Short list of changes in 0.5.0:
- Updated from log 0.3 to log 0.4. Both are interoperable, but using
  log 0.4 provides a much cleaner log interface for fern to use
  internally
- Removed fern::FernLog.
- Renamed fern::color::ColoredLogLevelConfig to ColoredLevelConfig
- Greatly simplified testing

0.5.0 (2017-12-25)
==================

- Update from log 0.3 to log 0.4. Both log versions are interoperable,
  but line numbers from libraries using 0.4 won't show up in binaries
  with recievers using log 0.4.
  - To clarify: both fern 0.4 and 0.5 will work perfectly with all
    libraries, but line numbers will always be 0 if you use fern 0.4
     and log 0.4.
- Remove fern::FernLog. With log 0.4, log records can be constructed
  directly, and fern loggers can now recieve just a record rather than
  a record and the formatted display string.
- Notable changes in the log crate: log::LogLevel is renamed to
  log::Level, and log::LogLevelFilter is renamed to log::LevelFilter.
- fern::color::ColoredLogLevelConfig has been renamed to
  fern::color::ColoredLevelConfig to match log crate renamings.
- fern tests have been greatly simplified with the new support for
  creating log records manually. it's now possible to just run
  "cargo test" and test all of fern's functionality.

0.4.4 (2017-12-22)
==================

- Add support for coloring log levels in Unix terminals using the
  'colored' crate
  - This is enabled via the 'colored' feature, and adds a fern::color
    module.

0.4.3 (2017-09-20)
==================

- Add support for sending to an std::sync::mpsc::Sender as a log output

0.4.2 (2017-08-20)
==================

- Documentation hotfix after a premature release of version 0.4.1

0.4.1 (2017-08-20)
==================

- Lots of documentation tweaks and reworking
- Add CONTRIBUTING file and update README to invite new contributors
- Improve example application to be more realistic
- A few small internal improvements, mostly code cleanup here

0.4.0 (2017-05-09)
==================

- Rework API surface to be builder-based for simpler configuration
  - Rename DispatchConfig to Dispatch, OutputConfig to Output and
    FernLogger to FernLog

- Rework inner log structure for more efficiency
  - Different outputs are now stored in an `enum` rather than every
    sublogger being a Box<FernLog> with dynamic dispatch
  - Remove LogError; handle errors within individual loggers now - and
    only within loggers which actually need it
  - Remove unnecessary wrapping of streams with an Arc (now just uses
    Mutex for File streams)
  - Remove unnecessary wrapping of Stdout and Stderr streams with a
    Mutex, when they are already synchronized
  - Pass around just &fmt::Arguments + &log::LogRecord instead of
    passing each individual LogRecord part

- Move opening of files and stdout/stderr from configuration
  "building" to configuring
  - Instead of taking OpenOptions, log configuration now just takes an
    already-opened std::io::File object
  - fern::InitError is now a convenience API, and is never returned
    from any fern APIs

- Redo formatting to work without allocation - formatting closures now
  finish with a callback rather than returning a value
- Update examples to use `chrono` instead of the `time` crate
  - This removes another extra allocation - chrono can format time
    directly to a writer, without allocating intermediate the result
    to a String

- Add much more documentation: almost every builder method has a full
  example, and all features should be thoroughly explained
- Add appveyor and travis-ci badges to README and Cargo.toml

0.3.5 (2015-05-06)
==================

- Build changes to .travis.yml
- Add html_root_url doc attribute
- Add file_with_line_sep and file_with_options_and_line_sep
  configuration construction options to allow specifying a line
  separator other than the default '\n'

0.3.4 (2015-04-16)
==================

- Update for rustc version e9080ec39 (1.0.0-beta.2)
  - Update to use no_test to ignore doc tests, rather than ignore
  - Remove all stability attributes on public types
  - Add rust version matrix for testing on travis, to test on beta as
    well as nightly builds

0.3.3 (2015-04-03)
==================

- Update for rustc version 9854143cb (1.0.0-beta)
  - Derive Clone for all types deriving Copy
  - Update docs a bit for that switch to `time` crate
- Switch to time crate instead of chrono for tests, as chrono hasn't
  updated for rustc 1.0.0-beta yet.
- Instead of implementing a sudo-time crate as a workaround for
  https://github.com/rust-lang/cargo/issues/1474, just disable the doc
  test, and copy the code to a separate file in tests/

0.3.2 (2015-04-03)
==================

- Update to rustc version 2e3b0c051
  - Add a workaround for
    https://github.com/rust-lang/cargo/issues/1474 in doc tests
  - Implement From for errors instead of FromError
  - Remove now unrequired feature gate
- Implement error::Error for error types

0.3.1 (2015-03-26)
==================

- Updates to rustc version 27901849e

0.3.0 (2015-03-25)
==================

- Updates to rustc version 123a754cb
- Updates to log version 0.3.0
- Reworks fern::OutputConfig to be a struct with functions to
  construct configurations, rather than an enum with variants for each
  configuration.
  - This is a breaking change, as all constructors on
    fern::OutputConfig have been renamed from UpperCase to lower_case.
  - This also now allows fern::OutputConfig to be constructed with
    anything which implements `AsRef<path::Path>`.
    - For example, `fern::OutputConfig::file("some-file.log")` works,
      without having to construct a Path or PathBuf manually.

0.2.1 (2015-03-19)
==================

- Updates to rustc version 3e4be02b8
- Updates documentation

0.2.0 (2015-03-08)
==================

This version reworks the public API in order to turn fern into a
backend to the `log` crate.

API Changes:
- Remove the `local` module, as the `log` crate now handles storing a
  global logger.
- fern::Logger *must* now be Sync + Send
- BoxedLogger and ArcLogger typedefs are removed, due to writing `+
  Sync + Send` no longer being required
 - Now everything just uses Box<Logger>
- Level is removed, in favor of using log::LogLevel and
  log::LogLevelFilter
- LoggerConfig is renamed into DispatchConfig
- Rename `Error` to `LogError`
  - Implement `fmt::Display` for `LogError`
- A new `Formatter` type is added for formatting closures. It also now
  takes a &log::LogLocation parameters as well.
- OutputConfig::Parent is renamed into OutputConfig::Child, this seems
  to make more sense, given that you can have any number of children
- Logger::log() now takes (&str, &log::LogLevel, &log::LogLocation)
  instead of (&fern::Level, &str)
- Add an `IntoLog` trait which DispatchConfig and OutputConfig
  implement (instead of having `into_logger()` on each of them.
  - Add an `into_log()` method to the IntoLog trait that turns a log
    configuration into a `log::Log` (as apposed to `fern::Logger`)
  - Rename `IntoLog.into_logger()` to `IntoLog.into_fern_logger()` in
    order to differentiate from the `into_log()` method.
- Add a `fern::init_global_logger()` method which sets the global
  `log` crate logger from a log configuration
- Add an `InitError` error which is used by `init_global_logger()` for
  either an IO error or `log::SetLoggerError`
- Update everything to use the new io and path modules
- Add a `FileOptions` option to `OutputConfig` which allows for
  specifying an `OpenOptions` for opening the log file with

Additional Changes:
- The docs have been rewritten to be up to date with all the above
  changes
  - The code snippets in the docs are now all tested! This is instead
    of having `no_test` and not having fully workable code examples.
- There is a new `tests/lib.rs` file which contains tests for
  initializing fern and log filtering with different log levels.

0.1.12 (2015-02-21)
===================

- Fixes compile warnings and errors for rustc version 522d09dfe
 - Adds static life bound
 - Switches to using old_path feature instead of path feature

0.1.11 (2015-02-13)
===================

- Fixes small documentation error
- Fixes compile errors in rustc version 3ef8ff1f8

0.1.10 (2015-02-03)
===================

- Finishes updating to rustc version eaf4c5c78
  - Last version compiled, but had many warnings
  - Move all #[experimental] features to #[unstable]
  - Add #![feature(io, core)]
  - Remove unrequired .iter() call

0.1.9 (2015-02-03)
==================

- Updates to rustc version eaf4c5c78
  - Changes all usages of std::io to std::old_io

0.1.8 (2015-01-27)
==================

- Updates to rustc version 458a6a2f6

0.1.7 (2015-01-09)
==================

- Update to latest rustc (44a287e6e)

0.1.6 (2015-01-07)
==================

This update mainly just cleans stuff up and updates for the latest
rustc (ea6f65c5f)

- Update to using f.write_str(... instead of write!(f, "{}", ...) for
  simplicity
- Update to use (closure)(...) instead of closure.call((...)) because
  directly calling works now
- Remove #![feature()] attributes for unboxed_closures and
  old_orphan_check, as they are no longer required.

0.1.5 (2015-01-05)
==================

- Updates for the latest rustc version, ad9e75938.
- Fixes all lines which go past the 99 character line limit.


0.1.4 (2015-01-01)
==================

This version is *not* backwards compatible. The change was necessary
for the latest rust update however, so only a minor version increment
was added.

- Changes from using IoResult<()> to Result<(), fern::Error> for
  return types from logging operations.
- Updates for latest rustc

0.1.3 (2014-12-27)
==================

- Adds a new public module, local, which stores a thread-local logger.
- Adds a new logger 'NullLogger', which does nothing with logged
  mesages.
- Fixes WriterLogger to append to opened files instead of overwriting.
- Adds a ton more documentation

0.1.2 (2014-12-24)
==================

- Adds type aliases BoxedLogger and ArcLogger, which resolve to
  `Box<Logger + Sync + Send>` and
  `sync::Arc<Box<Logger + Sync + Send>>` respectively.

0.1.1 (2014-12-22)
==================

- Adds a workaround for a bug introduced a compiler update.

0.1.0 (2014-12-19)
==================

First release, version 0.1.0.