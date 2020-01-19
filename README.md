# xtp: A rust binding for [XTP SDK](http://xtp.zts.com.cn) ![travis] [![docs.rs badge]][docs.rs link] [![Latest Version]][crates.io] 

`xtp` is a Rust binding for the [XTP SDK](http://xtp.zts.com.cn),
which is an interface for trading on the Chinese stock market.

## Current status

All APIs are implemented but are partially tested/documented. USE AT YOUR OWN RISK!

## Dependencies

* This crate depends on `libxtpquoteapi` and `libxtptraderapi` from the [**bundled** XTP SDK](http://xtp.zts.com.cn). Please install them first.
(The **Installation** here means adding the path to the correponding libs to `LD_LIBRARY` on linux, or `%PATH%` on windows.)
The version of the installed lib should be same as the one in the [bundled sdk](http://github.com/dovahcrow/xtp-sdk) in case of incompatibility.

* Only 64 bit of the SDK is implemented.

* For windows users, please install 64bit rust compiler and llvm (required by [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)).

## Usage

Please check out the [examples](https://github.com/dovahcrow/xtp-rs/tree/master/examples) folder for detail usage.



[travis]: https://img.shields.io/travis/dovahcrow/xtp-rs/master?style=flat-square
[docs.rs badge]: https://docs.rs/xtp/badge.svg
[docs.rs link]: https://docs.rs/xtp/
[crates.io]: https://crates.io/crates/xtp
[Latest Version]: https://img.shields.io/crates/v/xtp.svg