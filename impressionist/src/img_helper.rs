use image::{GenericImageView, Pixel, Rgb, RgbImage};
use imageproc::point::Point;
use itertools::izip;

pub fn bounding_box(points: &[Point<i32>]) -> (u32, u32, u32, u32) {
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;

    for p in &points[1..] {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);

        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    (min_x as u32, min_y as u32, width as u32, height as u32)
}

#[allow(unused)]
pub fn subtract_images(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = img1.dimensions();
    let mut out_img = RgbImage::new(width, height);

    izip!(img1.pixels(), img2.pixels(), out_img.pixels_mut()).for_each(|(p1, p2, p_out)| {
        *p_out = p1.map2(p2, |a, b| a.abs_diff(b));
    });
    out_img
}

pub fn calculate_difference(img1: &RgbImage, img2: &RgbImage) -> u64 {
    img1.pixels()
        .zip(img2.pixels())
        .fold(0u64, |acc, (p1, p2)| {
            let p_out = p1.map2(p2, |a, b| a.abs_diff(b));
            let diff = p_out.0.iter().fold(0u64, |acc, x| acc + (*x as u64));
            acc + diff
        })
}

pub fn get_average_pixel<I>(img1: I) -> Rgb<u8>
where
    I: GenericImageView<Pixel = Rgb<u8>>,
{
    let (w, h) = img1.dimensions();
    let pixel_count = w as usize * h as usize;

    if pixel_count == 0 {
        return Rgb([0, 0, 0]);
    }

    // Wir entpacken das Tupel (x, y, pixel) zu (_, _, item)
    let rgb_sum = img1.pixels().fold([0usize; 3], |acc, (_, _, item)| {
        [
            acc[0] + item[0] as usize,
            acc[1] + item[1] as usize,
            acc[2] + item[2] as usize,
        ]
    });

    // Direktes Erzeugen des Arrays vermeidet Vec-Allokation
    Rgb([
        (rgb_sum[0] / pixel_count) as u8,
        (rgb_sum[1] / pixel_count) as u8,
        (rgb_sum[2] / pixel_count) as u8,
    ])
}
#[cfg(test)]
mod tests {
    use crate::img_helper::calculate_difference;

    use super::*;

    fn flatten_pixels(input: &[[u8; 3]]) -> Vec<u8> {
        input.into_iter().copied().flatten().collect::<Vec<u8>>()
    }

    #[test]
    fn test_add() {
        let pixel = flatten_pixels(&[[5, 6, 5], [5, 5, 5]]);
        let pixel2 = flatten_pixels(&[[5, 5, 7], [5, 5, 0]]);

        let img1 = RgbImage::from_raw(2, 1, pixel).unwrap();
        let img2 = RgbImage::from_raw(2, 1, pixel2).unwrap();
        let d = calculate_difference(&img1, &img2);
        assert_eq!(d, 8);
    }
}
