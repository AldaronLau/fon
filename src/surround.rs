// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Surround Sound 5.1 speaker configuration and types.

use crate::{
    chan::{Ch16, Ch32, Ch64, Ch8, Channel},
    sample::Sample,
};

/// Surround Sound 5.1 sample format (front left channel, rear left channel,
/// rear right, front right channel, center, lfe).
#[derive(Default, PartialEq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Surround<C: Channel> {
    channels: [C; 6],
}

impl<C: Channel> Surround<C> {
    /// Create a one-channel Sample.
    pub fn new<H>(one: H, two: H, three: H, four: H, five: H, six: H) -> Self
    where
        C: From<H>,
    {
        let channels = [
            C::from(one),
            C::from(two),
            C::from(three),
            C::from(four),
            C::from(five),
            C::from(six),
        ];
        Self { channels }
    }
}

impl<C: Channel> Sample for Surround<C> {
    const CONFIG: &'static [[f64; 2]] = &[
        [1.0 / 12.0, 0.25],         // Front Left (Centered at 1/6)
        [0.25, 0.5],                // Rear Left (Centered at 1/3)
        [0.5, 0.75],                // Rear Right (Centered at 2/3)
        [0.75, 11.0 / 12.0],        // Front Right (Centered at 5/6)
        [11.0 / 12.0, 13.0 / 12.0], // Front Center
        [f64::NAN, f64::NAN],       // LFE
    ];

    type Chan = C;

    fn channels(&self) -> &[Self::Chan] {
        &self.channels
    }

    fn channels_mut(&mut self) -> &mut [Self::Chan] {
        &mut self.channels
    }

    fn from_channels(ch: &[Self::Chan]) -> Self {
        Self::new::<C>(ch[0], ch[1], ch[2], ch[3], ch[4], ch[5])
    }
}

/// 5.1 Surround [8-bit PCM](../chan/struct.Ch8.html) format.
pub type Surround8 = Surround<Ch8>;
/// 5.1 Surround [16-bit PCM](../chan/struct.Ch16.html) format.
pub type Surround16 = Surround<Ch16>;
/// 5.1 Surround [32-bit Floating Point](../chan/struct.Ch32.html) format.
pub type Surround32 = Surround<Ch32>;
/// 5.1 Surround [64-bit Floating Point](../chan/struct.Ch64.html) format.
pub type Surround64 = Surround<Ch64>;
