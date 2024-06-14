#![feature(let_chains)]
#![feature(slice_index_methods)]
#![feature(seek_stream_len)]

mod common;
mod utils;
mod reader;
mod writer;

pub use reader::Reader;
pub use writer::Writer;
