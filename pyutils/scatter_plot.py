from sys import stdin
from decimal import Decimal
from functools import reduce
from typing import Literal

import polars as pl
import orjson as json
import pyecharts.options as opts
from pyecharts.charts import Scatter


from helpers import query_music_db, find_level, dx_rating


def calculate_dxrating(music: dict):
    music_id = music["musicId"]
    level_id = music["level"]
    ach = music["achievement"]
    music_info = query_music_db(music_id)
    if not music_info:
        return music | {"dxRating": 0}
    level = find_level(music_info, level_id)

    try:
        return music | {"dxRating": dx_rating(Decimal(level.pop()["difficulty"]), ach)}
    except IndexError as _:
        return music | {"dxRating": 0}


data = json.loads(stdin.buffer.read())
user_id = data["userId"]
music_list: list[dict[str, dict]] = data["userMusicList"]
musics = reduce(
    lambda a, b: a + b, (music["userMusicDetailList"] for music in music_list)
)

musics = list(map(calculate_dxrating, musics))

df = (
    pl.LazyFrame(musics)
    .filter(pl.col("dxRating") > 0)  # filter out invalid play
    .select(["playCount", "dxRating"])
    .sort("dxRating", descending=False)
    .with_columns(pl.col("playCount").cum_sum())
    .collect()
)

x_data = df["playCount"].to_list()
y_data = df["dxRating"].to_list()


def init_chart(
    x_type: Literal["value", "log"],
    x_min: int = 1,
    x_max: int = 5000,
    y_max: int = 330,
) -> Scatter:
    return (
        Scatter(
            init_opts=opts.InitOpts(
                width="1600px",  # 设置图表宽度
                height="1000px",  # 设置图表高度
            )
        )
        .set_series_opts()
        .set_global_opts(
            xaxis_opts=opts.AxisOpts(
                type_=x_type,
                min_=x_min,
                max_=x_max,
            ),
            yaxis_opts=opts.AxisOpts(
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


x_max = (x_data[-1] // 50 * 50) + 50
y_max = min(330, (y_data[-1] // 50 * 50) + 50)

init_chart(
    "value",
    1,
    x_max,
    y_max=y_max,
).render(f"plot_cache/{user_id}-pc-rating-linear.html")
init_chart(
    "log",
    (x_max // 100) or 10,
    x_max,
    y_max=y_max,
).render(f"plot_cache/{user_id}-pc-rating-log.html")
