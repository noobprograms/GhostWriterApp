use image::{Rgba, RgbaImage};
use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::draw_text_mut;

use crate::video_process::script::Line;

pub fn render_frame(
    img: &mut RgbaImage,
    lines: &[Line],
    current_time: f32,
    font: &FontRef,
    draw_glow: &bool,
) {
    for line in lines {
        println!("Checking line '{}' at time {:.2}s against current time {:.2}s", line.text, line.time, current_time);
        if current_time >= line.time {
            let progress = ((current_time - line.time) / 2.0).clamp(0.0, 1.0);

            let visible_len =
                (line.text.len() as f32 * progress) as usize;

            let text = &line.text[..visible_len.min(line.text.len())];

            draw_glow_text(img, text, 50, 100, font, draw_glow);
        }
    }
}

fn draw_glow_text(
    img: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    font: &FontRef,
    draw_glow: &bool,
) {
    let scale = PxScale::from(42.0);
    println!("Drawing text '{}' at position ({}, {}) with scale {}", text, x, y, scale.x);
    // glow layers (fake blur)
    //we should be able to turn this on and off with a setting in the future
    if *draw_glow {
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
    println!("Text drawn successfully.");
}