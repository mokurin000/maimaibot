import asyncio
from sys import argv, stderr
from os.path import abspath
from base64 import b64decode

from playwright.async_api import async_playwright


async def render(
    html_path: str,
    png_path: str,
    transparent: bool = False,
):
    async with async_playwright() as pl:
        browser = await pl.chromium.connect("ws://127.0.0.1:51145")
        page = await browser.new_page(viewport={"width": 1600, "height": 1000})
        await page.goto(f"file://{abspath(html_path)}")

        await page.locator("div.echarts-finished").wait_for(state="attached")

        if transparent:
            png_base64 = await page.evaluate(
                """
        echarts.getInstanceByDom(
            document.querySelector("div[_echarts_instance_]")
        ).getDataURL({
            type: 'png',
            pixelRatio: 1,
            excludeComponents: ['toolbox']
        }).split(',')[1]
    """
            )

            with open(png_path, "wb") as f:
                f.write(b64decode(png_base64))
        else:
            await page.screenshot(path=png_path)

        await page.close()


async def main():
    match argv[1:]:
        case [html_path, png_path]:
            pass
        case _:
            print("usage: xxx.py <HTML> <PNG>", file=stderr)
            return
    render(
        png_path=png_path,
        html_path=html_path,
    )


if __name__ == "__main__":
    asyncio.run(main())
