//! Module containing renderers

use doc::Document;
use error::Result;

/// Trait representing all the functions a render system should have.
pub trait RenderSystem {
    /// Render to whatever target the render system has set.
    fn render(&self, doc: &Document) -> Result<()>;
}

/// The bundled/official/etc render system. Yay!
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct PdfRenderer {
    _force_private_construction: ()
}

impl PdfRenderer {
    /// Create a new pdf renderer
    pub fn new() -> PdfRenderer {
        PdfRenderer {
            _force_private_construction: ()
        }
    }
}

impl RenderSystem for PdfRenderer {
    fn render(&self, doc: &Document) -> Result<()> {
        panic!("unimplemented! (last value: {:?})", doc);
    }
}
