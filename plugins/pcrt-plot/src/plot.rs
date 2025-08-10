use std::error::Error;

use glam::U8Vec3;
use plotters::{
    coord::ranged1d::{AsRangedCoord, DefaultFormatting},
    prelude::*,
    style::full_palette::LIGHTBLUE,
};

fn colormap(rating: i32) -> RGBColor {
    // from zrender fastlerp
    let colors = [
        // "rgba(80, 163, 186, 1)",
        U8Vec3::new(0x50, 0xa3, 0xba).as_dvec3(),
        // "rgba(234, 199, 99, 1)",
        U8Vec3::new(0xea, 0xc7, 0x63).as_dvec3(),
        // "rgba(217, 78, 93, 1)",
        U8Vec3::new(0xd9, 0x4e, 0x5d).as_dvec3(),
    ];

    let t = (rating.abs() as f64 / 330.0) * (colors.len() - 1) as f64; // normalization
    let base_color = colors[t.floor() as usize];
    let bonus_color = colors[t.ceil() as usize];
    let bonus = t - t.floor();
    let U8Vec3 { x, y, z } = (base_color + bonus * (bonus_color - base_color)).as_u8vec3();

    RGBColor(x, y, z)
}
pub fn draw_chart(
    image_path: impl AsRef<std::path::Path>,
    xy_data: impl IntoIterator<Item = (i32, i32)>,
    x_min: i32,
    x_max: i32,
    y_max: i32,
    last_x: i32,
    is_log_x: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if is_log_x {
        let x_spec = (x_min..x_max).log_scale();
        draw_chart_impl(image_path, xy_data, x_spec, y_max, last_x)
    } else {
        let x_spec = x_min..x_max;
        draw_chart_impl(image_path, xy_data, x_spec, y_max, last_x)
    }
}

pub fn draw_chart_impl<X, R>(
    image_path: impl AsRef<std::path::Path>,
    xy_data: impl IntoIterator<Item = (i32, i32)>,
    x_spec: X,
    y_max: i32,
    last_x: i32,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
    X: AsRangedCoord<Value = i32, CoordDescType = R>,
    R: Ranged<FormatOption = DefaultFormatting, ValueType = i32>,
{
    let root = BitMapBackend::new(&image_path, (1600, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_range = 50..y_max;

    let mut chart = ChartBuilder::on(&root)
        .margin(40)
        .x_label_area_size(80)
        .y_label_area_size(75)
        .build_cartesian_2d(x_spec, y_range)?;

    let label_style = ("sans-serif", 23, &LIGHTBLUE).into_text_style(&root);
    let desc_style = ("sans-serif", 30, &BLACK).into_text_style(&root);
    let title_style = ("sans-serif", 40, &BLACK).into_text_style(&root);

    chart
        .configure_mesh()
        .y_labels(((y_max + 9) / 10) as _)
        .x_label_style(label_style.clone())
        .y_label_style(label_style.clone())
        .draw()?;
    // draw points
    chart.draw_series(xy_data.into_iter().map(|(x, y)| {
        Circle::new((x, y), 2.5, colormap(y).filled()) // 点大小为 5
    }))?;

    // add desc
    root.draw_text(&format!("Total play: {last_x}"), &desc_style, (100, 950))?;
    // add title
    root.draw_text(
        "Total Track/Difficulty Play - Single Track DX Rating",
        &title_style,
        (450, 940),
    )?;

    // render whole chart
    root.present()?;

    Ok(())
}
