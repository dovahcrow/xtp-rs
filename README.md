# xtp-rs: A rust binding (WIP) for [XTP SDK](http://xtp.zts.com.cn) ![travis]

This library is a binding for the [XTP SDK](http://xtp.zts.com.cn), which is an interface for trading on the Chinese stock market.

## Current status

**WIP**/**POC**

**!!!WARNING!!!**: Lots of functionalities are not implemented and the correctness of this binding is not verified. USE AT YOUR OWN RISK!

## Dependencies

This crate depends on `libxtpquoteapi` and `libxtptraderapi` from the **bundled** [XTP SDK](http://xtp.zts.com.cn). Please install them first. 
The version number should be same as the one in the [bundled sdk](http://github.com/dovahcrow/xtp-sdk) in case of incompatibility.

## Usage

This project is currently in its pre-Alpha stage and not being uploaded to crates.io yet. Use the git version if you really want to play with it.

Check the `examples` folder for usages.

[travis]: https://img.shields.io/travis/dovahcrow/xtp-rs/master?style=flat-square
