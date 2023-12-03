extern crate image;

use image::{DynamicImage, GenericImageView, imageops::FilterType};

fn main() {
    // 画像ファイルのパスを指定してください
    let image_path = "Your FILE PATH";

    // 画像を読み込む
    let img = image::open(image_path).expect("Failed to open image");

    // プロンプトの幅を指定
    let prompt_width = 80;

    // 高さを0.75倍にする
    let prompt_height_ratio = 0.75;
    let prompt_height = (prompt_height_ratio * prompt_width as f32) as u32;

    // ASCIIアートに変換
    let ascii_art = image_to_ascii(&img, prompt_width, prompt_height);

    // 結果を表示
    println!("{}", ascii_art);
}

fn image_to_ascii(img: &DynamicImage, prompt_width: u32, prompt_height: u32) -> String {
    // 画像を指定したサイズにリサイズ
    let img_resized = img.resize_exact(prompt_width, prompt_height, FilterType::Nearest);

    // ピクセルごとにASCII文字に変換
    let ascii_art: String = img_resized.pixels().enumerate().flat_map(|(i, p)| {
        let rgba = p.2;

        // RGBをグレースケールに変換
        let gray_value = (0.2126 * rgba[0] as f32 + 0.7152 * rgba[1] as f32 + 0.0722 * rgba[2] as f32) / 255.0;

        // グレースケールの明るさを利用してASCII文字に変換
        let ascii_char = intensity_to_ascii(gray_value);

        // 改行文字を挿入
        if i as u32 % prompt_width == 0 {
            vec![ascii_char, '\n']
        } else {
            vec![ascii_char]
        }
    }).collect();

    ascii_art
}

fn intensity_to_ascii(intensity: f32) -> char {
    // ASCII文字の範囲を設定
    let ascii_chars = "@%#*+=-:. ";

    // intensityを範囲に変換
    let intensity_range = (intensity * (ascii_chars.len() - 1) as f32).round() as usize;

    // 対応するASCII文字を取得
    ascii_chars.chars().nth(intensity_range).unwrap_or(' ')
}
