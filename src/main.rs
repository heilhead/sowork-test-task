mod data;
mod geom;

use crate::data::TestInput;
use crate::geom::{compute_rect_area, find_intersection};
// Anyhow helps convert between different error types and provides a bunch of usefule macros (not so useful
// for this task though...)
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Declare the arguments our program requires - `clap` will handle the parsing and the rest.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    filename: String,
}

fn read_test_input_from_file<P: AsRef<Path>>(path: P) -> Result<TestInput> {
    // Basic serde deserialization from file, copied straight from the docs.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let input = serde_json::from_reader(reader)?;
    Ok(input)
}

fn main() {
    // Parse arguments using `clap`. It'll actually handle argument help and will bail if required args are
    // not found.
    let args = Args::parse();

    // See if we manage to read the data. All kinds of errors should be handled here - i.e. file
    // inaccessible, malformed data etc.
    let result = read_test_input_from_file(&args.filename);
    if !result.is_ok() {
        println!("Error reading input data: {}", result.err().unwrap());
        return;
    }

    let input = result.unwrap();

    println!("Input data:");

    // Print the objects we found in the json file.
    for obj in &input.objects {
        println!(
            "Object name: {} Rect: {:?} Area: {}",
            obj.name,
            obj.rect,
            compute_rect_area(&obj.rect)
        );

        // Pretty simple nested loop to find what other objects it intersects.
        for intersect_obj in &input.objects {
            // Make sure we're not finding intersection with self.
            if std::ptr::eq(intersect_obj, obj) {
                continue;
            }

            if let Some(intersection) = find_intersection(&obj.rect, &intersect_obj.rect) {
                println!(
                    "\tIntersection: Object name: {} Rect: {:?} Area: {}",
                    intersect_obj.name,
                    intersection,
                    compute_rect_area(&intersection)
                );
            }
        }
    }
}
