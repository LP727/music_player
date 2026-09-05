//! # Audio
//!
//! This module provides an interface for the Sink/OutputStream wrapping

use std::fs::File;
use std::path::PathBuf;

use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

pub fn is_supported_format(path: &PathBuf) -> bool {
    let mut hint = Hint::new();

    // Provide the file extension as a hint.
    if let Some(extension) = path.extension() {
        if let Some(extension_str) = extension.to_str() {
            hint.with_extension(extension_str);
        }
    }
    let source = match File::open(path) {
        Ok(file) => Box::new(file),
        _ => return false,
    };
    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(source, Default::default());

    // Use the default options for format readers other than for gapless playback.
    let fmt_opts: FormatOptions = Default::default();

    // Use the default options for metadata readers.
    let meta_opts: MetadataOptions = Default::default();

    match symphonia::default::get_probe().probe(&hint, mss, fmt_opts, meta_opts) {
        Ok(_) => return true,
        _ => return false,
    }
}
