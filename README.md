# paranoid-android

Integration layer between `tracing` and Android logs

[![Crates.io](https://img.shields.io/crates/v/paranoid-android?style=flat-square)](https://crates.io/crates/paranoid-android) [![docs.rs](https://img.shields.io/docsrs/paranoid-android?style=flat-square)](https://docs.rs/paranoid-android)

This crate provides a [`MakeWriter`](https://docs.rs/tracing-subscriber/0.3/tracing_subscriber/fmt/trait.MakeWriter.html) suitable for writing Android logs.

It is designed as an integration with the [`fmt`](https://docs.rs/tracing-subscriber/0.3/tracing_subscriber/fmt/index.html) subscriber from `tracing-subscriber` and as such inherits all of its features and customization options.
