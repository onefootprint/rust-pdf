# pdf-canvas
A pure rust library for generating PDF files.
Currently, simple vector graphics and text set in the 14 built-in
fonts are supported.

[![Build Status](https://travis-ci.org/kaj/rust-pdf.svg?branch=master)](https://travis-ci.org/kaj/rust-pdf)
[![Crate](https://meritbadge.herokuapp.com/pdf-canvas)](https://crates.io/crates/pdf-canvas)
[![Build status](https://ci.appveyor.com/api/projects/status/vwtbgqkdf0rki6rd/branch/master?svg=true)](https://ci.appveyor.com/project/kaj/rust-pdf/branch/master)

To use this library, add it as a dependency in your `Cargo.toml`:

    [dependencies]
    pdf-canvas = "*"

The API is still very alpha, usage may change.
Some examples, that should work with the version containing them, can
be found in the [examples](examples) directory.
Read the
[API documentation for the pdf-canvas crate](https://docs.rs/pdf-canvas).
A larger example of a program using this crate is
[chord3, a chopro formatter](https://github.com/kaj/chord3).
