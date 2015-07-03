//! A rust library for pdfs.
//! It's pretty cool, I think.

// Lint heavily
#![allow(dead_code)]
#![warn(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]

// Dev note: Order using statements in:
//   super, crate, std, external crates, fully qualified

mod catalog;
mod contents;
mod document;
mod font;
mod objects;
mod outlines;
mod output;
mod page;
mod page_mode;
mod pages;
mod procedure;
mod rectangle;
mod resource_pool;
mod resources;
mod rotation;
mod version;

pub mod default_fonts;
pub mod default_dimensions;

pub use contents::Contents;
pub use contents::TextContents;
pub use document::Document;
pub use font::Font;
pub use output::Output;
pub use page::Page;
pub use pages::Pages;
pub use procedure::ProcSet;
pub use resource_pool::FontId;
