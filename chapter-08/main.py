from mandelbrot import mandelbrot_func
import asyncio


async def mandelbrot(size: int, path: str, range_x0: float, range_y0: float, range_x1: float, range_y1: float):
    mandelbrot_func(size, path, range_x0, range_y0, range_x1, range_y1)
    print(f"{path} created")

async def main():
    await asyncio.gather(*[
        mandelbrot(1000, f"purp{i}.png", -5.0, -2.12, -2.5, 1.12)
        for i in range(0,8)
    ])

asyncio.run(main())
