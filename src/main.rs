extern crate imgfilters;
extern crate tiny_tiff;

use imgfilters::median2d;
use tiny_tiff::reader;
use tiny_tiff::writer;

fn main() {
    // read
    let tiff = reader::open("./cell32.tif").unwrap();
    let bits = reader::bits_per_sample(tiff, 0);
    let width = reader::width(tiff);
    let height = reader::height(tiff);
    let size = width * height;
    let buffer: Vec<f32> = vec![0f32; size];
    reader::sample_data(tiff, &buffer, 0);
    reader::close(tiff);

    // make a 2d vec
    let mut buffer2d: Vec<Vec<f32>> = Vec::new();
    for chunk in buffer.chunks(width) {
        buffer2d.push(Vec::from(chunk));
    }

    // manipulate
    let filtered = median2d(buffer2d, 3, 3);
    let filtered: Vec<f32> = filtered.into_iter().flatten().collect();

    // write
    let tiff = writer::open("./cell32_filtered.tif", bits, width, height).unwrap();
    writer::write_image_float(tiff, &filtered);
    writer::close(tiff, "cell32 filtered!");
}
