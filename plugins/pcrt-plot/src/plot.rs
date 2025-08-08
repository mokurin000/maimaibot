use charming::{
    Chart, ImageFormat, ImageRenderer,
    component::{Axis, Title, VisualMap, VisualMapChannel, VisualMapType},
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{AxisTick, AxisType, Color, SplitLine},
    series::Scatter,
};
/// pc_rating must be non empty!
pub fn draw_webp(pc_rating: impl AsRef<[(i32, i32)]>) -> Result<Vec<u8>, charming::EchartsError> {
    let chart = make_chart(pc_rating).unwrap();
    let mut render = ImageRenderer::new(1600, 1000);
    render.render_format(ImageFormat::WebP, &chart)
}

fn make_chart(pc_rating: impl AsRef<[(i32, i32)]>) -> Option<Chart> {
    let pc_rating = pc_rating.as_ref();
    let x_max = pc_rating.last()?.0 / 50 * 50 + 50;
    let y_max = (pc_rating.last()?.1 / 50 * 50 + 50).min(330);

    let df: DataFrame = pc_rating
        .into_iter()
        .cloned()
        .map(|(x, y)| DataPoint::Value(CompositeValue::Array(vec![x.into(), y.into()])))
        .collect();

    Some(
        Chart::new()
            .background_color("#FFFFFF")
            .animation(false)
            .color(
                [
                    "#5470c6", "#91cc75", "#fac858", "#ee6666", "#73c0de", "#3ba272", "#fc8452",
                    "#9a60b4", "#ea7ccc",
                ]
                .map(Color::from)
                .to_vec(),
            )
            .visual_map(
                VisualMap::new()
                    .min(0)
                    .max(330)
                    .in_range(
                        VisualMapChannel::new()
                            .color(["#50a3ba", "#eac763", "#d94e5d"].map(Color::from).to_vec()),
                    )
                    .type_(VisualMapType::Continuous),
            )
            .x_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .name("总游玩曲目-次数")
                    .min(1)
                    .max(x_max),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Value)
                    .axis_tick(AxisTick::new().show(true))
                    .split_line(SplitLine::new().show(true))
                    .min(50)
                    .max(y_max),
            )
            .series(Scatter::new().symbol_size(5).data(df))
            .title(
                Title::new()
                    .text("pc v.s. rating")
                    .subtext("只计入最高A以上rank的曲目，按游玩track总数"),
            ),
    )
}
