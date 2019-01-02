pub fn median2d(img: Vec<Vec<f32>>, win_width: usize, win_height: usize) -> Vec<Vec<f32>> {
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
