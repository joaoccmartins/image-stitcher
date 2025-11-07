use clap::Parser;
use stitchy_core::{AlignmentMode, ImageFiles, FilePath, Stitch};

#[derive(Parser)]
struct Cli{
    images: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let mut builder = ImageFiles::builder();
    for img_path in args.images{
        builder = match builder.add_file(FilePath::new(img_path)) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to add directory: {}", e);
                return;
            }
        };
    }
    let built = builder.build();
    match built {
        Ok(images) => {
            let stitcher = Stitch::builder().image_files(images);
            match stitcher {
                Ok(mut stitcher) => {
                    stitcher = stitcher
                        .alignment(AlignmentMode::Horizontal);
                    let result = stitcher.stitch();
                    match result {
                        Ok(final_image) => {
                            let save_result = final_image.save("stitched_output.png");
                            match save_result {
                                Ok(_) => println!("Stitched image saved as stitched_output.png"),
                                Err(e) => eprintln!("Failed to save stitched image: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Error during stitching: {}", e),
                    }
                }
                Err(e) => eprintln!("Error creating stitcher: {}", e),
            }
        },
        Err(e) => eprintln!("Error building image files: {}", e),
    }
}
