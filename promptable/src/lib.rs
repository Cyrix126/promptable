#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
pub extern crate promptable_derive;
/// module for trait Inspectable.
/// this module activable with feature "inspect" will allow types with Inspectable trait to be inspected by the user with menus.
#[cfg(feature = "inspect")]
pub mod inspect;

#[cfg(feature = "basics")]
/// All the basics of the crate.
pub mod basics;
#[cfg(any(feature = "basics", feature = "anyhow"))]
pub extern crate anyhow;
#[cfg(feature = "basics")]
pub extern crate derive_more;
#[cfg(feature = "basics")]
pub extern crate inquire;
#[cfg(feature = "basics")]
pub extern crate termion;
#[cfg(feature = "basics")]
pub extern crate trait_gen;
