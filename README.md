# rust-pdf
A pure rust library for generating PDF files.
Currently, simple vector graphics and text set in the 14 built-in
fonts are supported.

[![Build Status](https://travis-ci.org/kaj/rust-pdf.svg?branch=master)]
(https://travis-ci.org/kaj/rust-pdf)
[![](http://meritbadge.herokuapp.com/pdf)](https://crates.io/crates/pdf)

To use this library, add it as a dependency in your `Cargo.toml`:

    [dependencies]
    pdf = "*"

The API is still very alpha, usage may change.
Some examples, that should work with the version containing them, can
be found in the [examples](examples) directory.
Read the [API documentation for the pdf crate]
(https://rasmus.krats.se/doc/pdf/0.4.0/pdf/).
A larger example of a program using this crate is [chord3, a chopro
formatter](https://github.com/kaj/chord3).
