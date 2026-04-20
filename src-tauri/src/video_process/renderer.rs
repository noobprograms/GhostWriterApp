use image::{Rgba, RgbaImage};
use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::draw_text_mut;

use crate::video_process::script::Line;

pub fn render_frame(
    img: &mut RgbaImage,
    lines: &[Line],
    current_time: f32,
    font: &FontRef,
) {
    for line in lines {
        if current_time >= line.time {
            let progress = ((current_time - line.time) / 2.0).clamp(0.0, 1.0);

            let visible_len =
                (line.text.len() as f32 * progress) as usize;

            let text = &line.text[..visible_len.min(line.text.len())];

            draw_glow_text(img, text, 50, 100, font);
        }
    }
}

fn draw_glow_text(
    img: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    font: &FontRef,
) {
    let scale = PxScale::from(42.0);

    // glow layers (fake blur)
    for offset in [-2, -1, 1, 2] {
        draw_text_mut(
            img,
            Rgba([255, 255, 255, 40]),
            x + offset,
            y,
            scale,
            font,
            text,
        );

        draw_text_mut(
            img,
            Rgba([255, 255, 255, 40]),
            x,
            y + offset,
            scale,
            font,
            text,
        );
    }

    // main text
    draw_text_mut(
        img,
        Rgba([255, 255, 255, 255]),
        x,
        y,
        scale,
        font,
        text,
    );
}