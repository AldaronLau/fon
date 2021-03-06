// Fon
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Rust audio types and conversions.
//!
//! An [audio buffer] can be cheaply converted to and from raw samples (i8, i16,
//! f32, and f64) buffers, enabling interoperability with other crates.
//!
//! Many audio formats are supported:
//! - Any sample rate
//! - Bit depth: [8]- or [16]-bit integer and [32]- or [64]-bit float
//! - [Mono], [Stereo], [5.1 Surround]
//!
//! Blending [operations] are supported for all formats.
//!
//! # Getting Started
//! 
//! To understand some of the concepts used in this library,
//! [this MDN article] is a good read (although the stuff about compression
//! isn't relevant to this crate's functionality).  This crate uses the MDN
//! definitions for what an audio frame and audio channel are.
//!
//! ## 8-Bit Sawtooth Wave Example
//! ```rust
//! use fon::chan::Ch8;
//! use fon::mono::Mono8;
//! use fon::stereo::Stereo16;
//! use fon::{Audio, Frame};
//!
//! let mut a = Audio::<Mono8>::with_silence(44_100, 256);
//! for (i, s) in a.iter_mut().enumerate() {
//!     s.channels_mut()[0] = Ch8::new(i as i8);
//! }
//! // Convert to stereo 16-Bit 48_000 KHz audio format
//! let audio = Audio::<Stereo16>::with_stream(48_000, &a);
//! ```
//!
//! [audio buffer]: crate::Audio
//! [8]: crate::chan::Ch8
//! [16]: crate::chan::Ch16
//! [32]: crate::chan::Ch32
//! [64]: crate::chan::Ch64
//! [Mono]: crate::mono::Mono
//! [Stereo]: crate::stereo::Stereo
//! [5.1 Surround]: crate::surround::Surround
//! [operations]: crate::ops
//! [this MDN article]: https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Audio_concepts

// FIXME: Doesn't quite work yet because fon needs sine and cosine.  No_std trig
// crate would be nice for a fon feature enabling no_std, but I couldn't find
// one.
//#![no_std]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg",
    html_root_url = "https://docs.rs/fon"
)]
// #![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

extern crate alloc;

mod audio;
pub mod chan;
mod frame;
mod math;
pub mod mono;
pub mod ops;
mod private;
pub mod stereo;
mod streaming;
pub mod surround;
// mod resampler;

pub use audio::Audio;
pub use frame::Frame;
pub use streaming::{Resampler, Sink, Stream};
