extern crate tiny_tiff;

use tiny_tiff::reader;
use tiny_tiff::writer;

fn median2d(img: Vec<Vec<f32>>, win_width: usize, win_height: usize) -> Vec<Vec<f32>> {
    let mut out = img.clone();

    let img_height = img[0].len();
    let img_width = img.len();

    let mut window = vec![0f32; win_width * win_height];

    let edgex = win_width / 2;
    let edgey = win_height / 2;

    for x in edgex..(img_width - edgex) {
        for y in edgey..(img_height - edgey) {
            let mut i = 0;
            for fx in 0..win_width {
                for fy in 0..win_height {
                    window[i] = img[x + fx - edgex][y + fy - edgey];
                    i += 1;
                }
            }
            window.sort_by(|a, b| a.partial_cmp(b).unwrap());
            out[x][y] = window[win_width * win_height / 2];
        }
    }
    out
}

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
