use zed_extension_api as zed;

struct CramExtension;

impl zed::Extension for CramExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(CramExtension);
