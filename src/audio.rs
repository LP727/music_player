//! # Audio
//!
//! This module provides an interface for the Sink/OutputStream wrapping

use std::fs::File;
use std::path::Path;

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Verifies if a specific file is supported by symphonia since it handles the codecs.
/// Note: To avoid mismatch in support, we keep Symphonia's version do the one rodio uses (0.5.5
/// currently)
///
/// # Arguments
///
/// - `path` (`&Path`) - Source (audio) file
///
/// # Returns
///
/// - `bool` - Format supported or not in rodio's version of symphonia
///
/// # Examples
///
/// TODO : Fill with AI
/// ```
/// use crate::...;
///
/// let _ = is_supported_format();
/// ```
pub fn is_supported_format(path: &Path) -> bool {
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

    // Use the default options for format readers other than for gapless
    // playback.
    let fmt_opts: FormatOptions = Default::default();

    // Use the default options for metadata readers.
    let meta_opts: MetadataOptions = Default::default();

    // if ok return true, is error return false
    symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .is_ok()
}
