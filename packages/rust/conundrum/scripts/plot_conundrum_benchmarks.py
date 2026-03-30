import os
from pathlib import Path
import json
import plotly.express as px
import plotly.io as pio
from dataclasses import dataclass
import re


pio.templates.default = "plotly_dark"


root = os.environ["FLUSTER_IOS_ROOT"]

if root is None:
    print(
        "Cannot continue without the FLUSTER_IOS_ROOT variable set to the root of the monorepo"
    )


p = Path(root) / "docs" / "generated" / "benchmark_data.json"

s = p.read_text()

data = json.loads(s)


@dataclass
class BenchmarkRun:
    id: str
    tested_on: float
    mean: float
    median: float
    slope: float | None
    change_mean: float | None
    change_median: float | None


PARSE_DATA = {"id": "parse_mdx_by_regex_50", "y_axis": "mean", "include_regex": "all"}

items = []


def get_item_change(item, val: str):
    if item["change"]:
        return item["change"][val]["estimate"]
    return None


def matches(item) -> bool:
    if PARSE_DATA["include_regex"]:
        return item["id"].__contains__(PARSE_DATA["include_regex"])
    else:
        return item["id"] == PARSE_DATA["id"]


for item in data:
    if matches(item) or PARSE_DATA["include_regex"] == "all":
        items.append(
            BenchmarkRun(
                item["id"],
                item["testedOn"],
                item["mean"]["estimate"],
                item["median"]["estimate"],
                item["slope"]["estimate"] if item["slope"] is not None else None,
                get_item_change(item, "mean"),
                get_item_change(item, "median"),
            )
        )

y = [x.__getattribute__(PARSE_DATA["y_axis"]) for x in items]

x = [x.tested_on for x in items]
fig = px.scatter(title=f"Conundrum: {PARSE_DATA['id']}", x=x, y=y)

fig.update_yaxes(title_text=PARSE_DATA["y_axis"])
fig.update_xaxes(title_text="time")

fig.show(renderer="browser")
