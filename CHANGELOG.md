# Changelog

All notable changes to this project will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project tries to adhere to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Release 0.7.0

2022-02-07 21:14:40 +02:00

* Merge PR #7 from @hummingly:
  - Updates dependencies.
  - Fixes a bug where adding keywords would just overwrite/add the Subject
    entry.
  - Changes create_X functions to constructors that are only visible in the
    crate.
  - Improves allocations: removed a few clones, allocate space before
    inserting elements etc.
  - Puts graphicsstates tests into test module which removes dead code
    warning.
  - Overall made more use of the standard library.
* Add keyword "pdf" to crate, PR #6 from @adiba
* Update to Rust edition 2021.
* Update `lazy_static` dependency.
* Use `chrono` 0.4.19 instead of `time` 0.1.
* Update travis build to use more up to date rust versions and stable
  rustfmt.
* Update clippy directive.
* Some rustfmt updates.

Thanks to @hummingly and @adiba!

## Release 0.6.0

2018-06-15 11:03:33 +02:00

* PR #5 from @hummingly: Adds ZapfDingbats encoding
* Fix Encoding::encode_string. The encoded bytes b'\', b'(', and b')' must
  be escaped properly, not the unencoded characters '\', '(', and ')'.
* Some documentation improvements.
* Remove some explict lifetimes.
* Avoid some cloning. Or at least delay it slightly, cloning in caller
  rather than callee.
* Code-style changes. Mainly use `x?` instead of `try!(x)` and follow
  rustfmt updates.
* Testing now also done on windows, by appveyour.

Thanks to @hummingly.


## Release 0.5.4

2017-02-15 00:12:11 +01:00

* Rename this crate to `pdf-canvas`.
* Update rust versions in CI to stable, beta, nightly, 1.14, and 1.13.
* Rustfmt update.

Before this release, the crate name was `pdf`.
That name was given to another project aiming for more general (read +
write) pdf support, while this was re-released as `pdf-canvas`.


## Release 0.5.0

2016-10-13 10:06:21 +0200.

* Improve `show_adjusted` api.
* Fix some missing encodings.
* Minor changes to makerelease and travis scripts.
* Add rust 1.11 to build, remove 1.8 and older.
* Use implicit deref.
* Some refactoring and cleanup, partially to match rustfmt updates.


## Release 0.4.2

2016-08-28 01:44:57 +02:00

* Improve documentation; everything that is public now has a docstring.
* Cleaned up the code extracing metrics from AFM files.
* Use `docs.rs` instead of `rasmus.krats.se` for documentation.
* Minor cleanup.


## Early history

Release 0.4.1 was made 2016-08-17 22:28:39 +02:00.
For history earlier than this, I give up.
Anyone interested is referred to the git log.

The initial commit was made 2015-09-24 15:35:02 +02:00 by @SimonSapin,
extracting code from another project, called
[robinson](https://github.com/SimonSapin/robinson/tree/pdf/pdf/).
