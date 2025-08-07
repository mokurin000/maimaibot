import asyncio
from sys import stdin
from typing import Literal

from sys import argv
import orjson as json
import pyecharts.options as opts
from pyecharts.charts import Scatter


from scnapshot_html import render


def init_chart(
    x_data: list[int],
    y_data: list[int],
    graph_type: Literal["value", "log"],
    x_min: int = 1,
    x_max: int = 5000,
    y_max: int = 330,
) -> Scatter:
    return (
        Scatter(
            init_opts=opts.InitOpts(
                width="1600px",  # 设置图表宽度
                height="1000px",  # 设置图表高度
                chart_id="scatter",
                animation_opts=opts.AnimationOpts(animation=False),
            )
        )
        # creates a `div` element to easily wait for 'finished'
        .add_js_funcs("""
chart_scatter.on('finished', () => {
    const elem = document.createElement("div");
    elem.setAttribute("class", "echarts-finished");
    document.querySelector('body').appendChild(elem);
});
""")
        .set_series_opts()
        .set_global_opts(
            title_opts=opts.TitleOpts(
                title="pc v.s. rating",
                subtitle="只计入最高A以上rank的曲目，按游玩track总数",
            ),
            xaxis_opts=opts.AxisOpts(
                name="总游玩曲目-次数",
                type_=graph_type,
                min_=x_min,
                max_=x_max,
            ),
            yaxis_opts=opts.AxisOpts(
                name="",
                type_="value",
                axistick_opts=opts.AxisTickOpts(is_show=True),
                splitline_opts=opts.SplitLineOpts(is_show=True),
                min_=50,
                max_=y_max,
            ),
            tooltip_opts=opts.TooltipOpts(is_show=False),
            visualmap_opts=opts.VisualMapOpts(max_=330),
        )
        .add_xaxis(xaxis_data=x_data)
        .add_yaxis(
            series_name="",
            y_axis=y_data,
            symbol_size=5,
            label_opts=opts.LabelOpts(is_show=False),
        )
    )


async def main():
    user_id = int(argv[1])

    data = json.loads(stdin.buffer.read())
    x_data = [d[0] for d in data]
    y_data = [d[1] for d in data]

    x_max = (x_data[-1] // 50 * 50) + 50
    y_max = min(330, (y_data[-1] // 50 * 50) + 50)

    output_html = f"plot_cache/{user_id}-pc-rating-linear.html"
    output_png = f"plot_cache/{user_id}-pc-rating-linear.png"

    gtype = "value"
    if gtype == "log":
        x_max = (x_max // 100) or 10
    init_chart(
        x_data,
        y_data,
        gtype,
        x_min=1,
        x_max=x_max,
        y_max=y_max,
    ).render(output_html)

    await render(output_html, output_png)


if __name__ == "__main__":
    asyncio.run(main())
