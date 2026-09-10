use anyhow::Result;
use image::DynamicImage;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Извлекает обложку альбома из аудиофайла или ищет рядом лежащий файл
pub fn extract_cover(path: &Path) -> Result<Option<DynamicImage>> {
    // Многие MP3/FLAC-файлы хранят обложку внутри тегов, без отдельного cover.jpg.
    if let Some(img) = extract_embedded_cover(path)? {
        return Ok(Some(img));
    }

    // Сначала ищем cover.jpg/cover.png в той же папке
    if let Some(parent) = path.parent() {
        let cover_files = [
            "cover.jpg",
            "cover.png",
            "folder.jpg",
            "album.jpg",
            "front.jpg",
            "cover.jpeg",
            "cover.webp",
            "folder.jpeg",
            "folder.webp",
            "front.jpeg",
            "front.webp",
        ];

        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let cover_path = entry.path();
                let Some(file_name) = cover_path.file_name().and_then(|name| name.to_str()) else {
                    continue;
                };
                if cover_files
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(file_name))
                {
                    if let Ok(img) = image::open(&cover_path) {
                        return Ok(Some(img));
                    }
                }
            }
        }

        // Ищем любое изображение в папке
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let p = entry.path();
                if is_image_file(&p) {
                    if let Ok(img) = image::open(&p) {
                        return Ok(Some(img));
                    }
                }
            }
        }
    }

    // TODO: Извлечение из метаданных аудиофайла
    Ok(None)
}

fn extract_embedded_cover(path: &Path) -> Result<Option<DynamicImage>> {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(None),
    };
    let source = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
        hint.with_extension(extension);
    }

    let mut probed = match symphonia::default::get_probe().format(
        &hint,
        source,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ) {
        Ok(probed) => probed,
        Err(_) => return Ok(None),
    };

    let mut format = probed.format;
    if let Some(mut metadata) = probed.metadata.get() {
        if let Some(visual) = metadata
            .skip_to_latest()
            .and_then(|revision| revision.visuals().first())
        {
            if let Ok(image) = image::load_from_memory(&visual.data) {
                return Ok(Some(image));
            }
        }
    }

    if let Some(visual) = format
        .metadata()
        .skip_to_latest()
        .and_then(|revision| revision.visuals().first())
    {
        if let Ok(image) = image::load_from_memory(&visual.data) {
            return Ok(Some(image));
        }
    }

    Ok(None)
}

fn is_image_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp" | "tiff" | "tif"
        ),
        None => false,
    }
}
