//! A basic example of a PDF file created with krilla.

use std::path;
use std::path::PathBuf;

use krilla::Document;
use krilla::geom::Point;
use krilla::page::PageSettings;
use krilla::text::Font;
use krilla::text::TextDirection;

fn main() {
    // Create a new document.
    let mut document = Document::new();
    // Load a font.
    let font = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/Ahem.ttf");
        let data = std::fs::read(&path)
            .unwrap_or_else(|_| panic!("Failed to read font data from '{}'", path.display()));

        Font::new(data.into(), 0).unwrap()
    };

    // Add a new page with dimensions 200x200.
    let mut page = document.start_page_with(PageSettings::new(200.0, 200.0));
    // Get the surface of the page.
    let mut surface = page.surface();
    // Draw some text.
    surface.draw_text(
        Point::from_xy(0.0, 25.0),
        font.clone(),
        14.0,
        "Hello, docspiler!",
        false,
        TextDirection::Auto,
    );

    // Finish the page.
    surface.finish();
    page.finish();

    let pdf = document.finish().unwrap();
    let path = path::absolute("basic.pdf").unwrap();
    eprintln!("Saved PDF to '{}'", path.display());

    // Write the PDF to a file.
    std::fs::write(path, &pdf).unwrap();
}
