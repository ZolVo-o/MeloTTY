use image::{DynamicImage, GenericImageView};
use ratatui::style::Color;

/// Конвертирует изображение в ASCII-арт с цветами терминала
pub fn image_to_ascii(img: &DynamicImage, width: u32, height: u32) -> Vec<(String, Vec<Color>)> {
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let mut lines = Vec::new();

    for y in 0..height {
        let mut line = String::new();
        let mut colors = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = pixel.0;

            // Пропускаем прозрачные пиксели
            if rgba[3] < 128 {
                line.push(' ');
                colors.push(Color::Rgb(0, 0, 0));
                continue;
            }

            let brightness =
                (0.299 * rgba[0] as f32 + 0.587 * rgba[1] as f32 + 0.114 * rgba[2] as f32) / 255.0;

            let char = if brightness > 0.9 {
                '█'
            } else if brightness > 0.8 {
                '▓'
            } else if brightness > 0.7 {
                '▒'
            } else if brightness > 0.6 {
                '░'
            } else if brightness > 0.5 {
                '▐'
            } else if brightness > 0.4 {
                '▌'
            } else if brightness > 0.3 {
                '▍'
            } else if brightness > 0.2 {
                '▎'
            } else if brightness > 0.1 {
                '▏'
            } else {
                ' '
            };

            line.push(char);
            colors.push(Color::Rgb(rgba[0], rgba[1], rgba[2]));
        }

        lines.push((line, colors));
    }

    lines
}
