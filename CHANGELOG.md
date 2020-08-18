# Changelog
All notable changes to `fon` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.2.0] - Unreleased
### Added
 - `From<Audio> for Box<[f64]>`
 - `From<Audio> for Box<[f32]>`
 - `From<Audio> for Box<[i8]>`
 - `From<Audio> for Box<[i16]>`

### Removed
 - `From<Audio> for Box<[u8]>`
 - `From<Audio> for Box<[u16]>`

## [0.1.0] - 2020-08-15
### Added
 - `Audio` buffer
 - `Hz` newtype
 - `Config` trait
 - `mono`, `stereo` and `surround` modules
 - `Sample1`, `Sample2`, `Sample6`, and `Sample8` implementing `Sample`
 - `ops` module with `Amplify`, `Clear`, `Compress`, `Dest`, `Mix` and `Src`
 - `Ch8`, `Ch16`, `Ch32` and `Ch64` implementing `Channel`