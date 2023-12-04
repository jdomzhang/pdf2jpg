use std::env;

use image::ImageFormat;
use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // Attempt to bind to a pdfium library in the current working directory; failing that,
    // attempt to bind to a system-provided library.

    // The library name will differ depending on the current platform. On Linux,
    // the library will be named libpdfium.so by default; on Windows, pdfium.dll; and on
    // MacOS, libpdfium.dylib. We can use the Pdfium::pdfium_platform_library_name_at_path()
    // function to append the correct library name for the current platform to a path we specify.

    let bindings = Pdfium::bind_to_library(
        // Attempt to bind to a pdfium library in the current working directory...
        Pdfium::pdfium_platform_library_name_at_path("./"),
    )
    .or_else(
        // ... and fall back to binding to a system-provided pdfium library.
        |_| Pdfium::bind_to_system_library(),
    )?;

    // check args
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <pdf file> <output folder>", args[0]);
        // return Ok(());
        // return error
        return Err(PdfiumError::UnrecognizedPath);
    }

    let pdf_file = &args[1];
    let output_folder = &args[2];

    // get file name without extension from pdf_file
    let file_name = pdf_file.split("/").last().unwrap();
    // remove extension
    let file_name = file_name.split(".").next().unwrap();

    // create folder if not existing
    std::fs::create_dir_all(output_folder).unwrap();

    // create pdfium
    let pdfium = Pdfium::new(bindings);

    // This pattern is common enough that it is the default constructor for the Pdfium struct,
    // so we could have also simply written:

    // let pdfium = Pdfium::default();

    // Next, we create a set of shared settings that we'll apply to each page in the
    // sample file when rendering. Sharing the same rendering configuration is a good way
    // to ensure homogenous output across all pages in the document.

    // let render_config = PdfRenderConfig::new()
    //     // .set_target_width(2000)
    //     // .set_target_width(1920 * 2);
    //     .set_target_width(1920);
    // // .set_maximum_width(1920 * 2)
    // // .set_maximum_height(1920 * 2);
    // // .set_maximum_height(2000);
    // // .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // Load the sample file...

    // let document = pdfium.load_pdf_from_file("test/export-test.pdf", None)?;
    let document = pdfium.load_pdf_from_file(pdf_file, None)?;

    // ... and export each page to a JPEG in the current working directory,
    // using the rendering configuration we created above.

    for (index, page) in document
        .pages()
        .iter() // ... get an iterator across all pages ...
        .enumerate()
    {
        let render_config = PdfRenderConfig::new().set_target_width(1920);
        // .set_target_height(page.height().value as i32)
        // .set_target_width(page.width().value as i32);

        let page_file = format!("{}/{}-{}.jpg", output_folder, file_name, index + 1);
        println!("{}", page_file);

        let result = page
            .render_with_config(&render_config)? // Initializes a bitmap with the given configuration for this page ...
            .as_image() // ... renders it to an Image::DynamicImage ...
            .as_rgba8() // ... sets the correct color space ...
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(
                page_file,
                // format!("./output/export-test-page-{}.jpg", index + 1),
                // format!("{}/{}-{}.jpg", output_folder, file_name, index + 1),
                ImageFormat::Jpeg,
            ); // ... and exports it to a JPEG.

        assert!(result.is_ok());
    }

    Ok(())
}
