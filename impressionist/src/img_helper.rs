use anyhow::Result;
use image::{ImageBuffer, ImageReader, Pixel, Rgb, RgbImage};
use imageproc::point::Point;
use itertools::izip;

pub fn bounding_box(points: &[Point<i32>]) -> (Point<i32>, Point<i32>) {
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

    let upper_left = Point { x: min_x, y: min_y };
    let lower_right = Point { x: max_x, y: max_y };
    (upper_left, lower_right)
}

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

pub fn get_average_pixel(img1: &RgbImage) -> Rgb<u8> {
    let pixel_count = img1.width() + img1.height();
    let rgb_sum = img1.pixels().fold([0usize; 3], |acc, item| {
        [
            acc[0] + item[0] as usize,
            acc[1] + item[1] as usize,
            acc[2] + item[2] as usize,
        ]
    });
    let avg = rgb_sum
        .iter()
        .map(|x| (x / pixel_count as usize) as u8)
        .collect::<Vec<_>>();
    let arr: [u8; 3] = avg.try_into().unwrap();
    Rgb(arr)
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
