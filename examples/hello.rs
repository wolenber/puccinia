extern crate puccinia;

use puccinia::*;

fn main() {
    let mut doc = Document::new();
    let font = doc.include_base_font(default_fonts::TIMES);
    doc.pages_mut().set_dimensions(default_dimensions::LETTER_PAGE);
    doc.append_page()
        .with_font(font)
        .with_procset(ProcSet::TextOnly)
        .use_contents()
            .as_text_contents()
            .set_font(font, 24.0)
            .displace(100.0, 100.0)
            .show_text("Hello, pdf!");
    println!("{}", doc.output());
}
