from PIL import Image


def mandelbrot(size: int, path: str, range_x0: float, range_y0: float, range_x1: float, range_y1: float):
    image = Image.new(mode='RGB', size=(size, size))

    size_f = float(size)

    x_range = abs(range_x1 - range_x0)
    x_offset = x_range / 2.0

    y_range = abs(range_y1 - range_y0)
    y_offset = y_range / 2.0

    for px in range(size):
        for py in range(size):
            x0 = float(px) / size_f * x_range - x_offset
            y0 = float(py) / size_f * y_range - y_offset

            c = complex(x0, y0)
            i = 0
            z = complex(0, 0)

            while i < 255: 
                z = (z * z) + c
                if float(z.real) > 4.0:
                    break
                i += 1

            image.putpixel((px, py), (i, i, i))
    image.save(path)

mandelbrot(1000, "purp.png", -5.0, -2.12, -2.5, 1.12)
