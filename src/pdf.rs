use printpdf::*;

pub enum PdfLibrary {
    PrintPdf,
    PdfCanvas,
}

pub fn generate_pdf(diff_type: &str, content: &str, library: PdfLibrary) {
    match library {
        PdfLibrary::PrintPdf => generate_pdf_printpdf(diff_type, content),
        PdfLibrary::PdfCanvas => {
            // TODO: Implement PDF generation with pdf_canvas
            unimplemented!()
        }
    }
}

fn generate_pdf_printpdf(diff_type: &str, content: &str) {
    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("Generated Diff PDF", Mm(247.0), Mm(297.0), "Layer 1");

    // Set up font
    let font = doc.add_external_font(File::open("assets/fonts/Roboto-Regular.ttf").unwrap()).unwrap();

    // Write diff type and content to the PDF
    let current_layer = doc.get_page(page1).get_layer(layer1);
    current_layer.set_font(&font, 16);
    current_layer.use_text(diff_type, 16, Mm(10.0), Mm(287.0), &doc.get_font(font).unwrap());
    current_layer.set_font(&font, 12);
    current_layer.use_text(content, 12, Mm(10.0), Mm(270.0), &doc.get_font(font).unwrap());

    // Save the PDF to a file
    doc.save(&mut BufWriter::new(File::create("output.pdf").unwrap())).unwrap();
}
