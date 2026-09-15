use image::{DynamicImage, GenericImageView};
use ratatui::style::Color;

/// Строка artwork: символы, цвет верхнего пикселя и цвет нижнего пикселя.
pub type Artwork = Vec<(String, Vec<Color>, Vec<Color>)>;

/// Преобразует изображение в плотный цветной artwork для терминала.
/// Один символ `▀` отображает два вертикальных пикселя изображения.
pub fn image_to_ascii(img: &DynamicImage, width: u32, height: u32) -> Artwork {
    let img = img.resize_exact(width, height * 2, image::imageops::FilterType::Lanczos3);
    let mut lines = Vec::new();

    for y in 0..height {
        let mut line = String::new();
        let mut foreground = Vec::new();
        let mut background = Vec::new();

        for x in 0..width {
            let top = img.get_pixel(x, y * 2).0;
            let bottom = img.get_pixel(x, y * 2 + 1).0;
            line.push('▀');
            foreground.push(pixel_color(top));
            background.push(pixel_color(bottom));
        }

        lines.push((line, foreground, background));
    }

    lines
}

fn pixel_color(pixel: [u8; 4]) -> Color {
    if pixel[3] < 128 {
        Color::Rgb(30, 30, 46)
    } else {
        Color::Rgb(pixel[0], pixel[1], pixel[2])
    }
}
