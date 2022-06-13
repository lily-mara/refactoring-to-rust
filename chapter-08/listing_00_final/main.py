import asyncio
import rust_async

async def thing(x):
    await rust_async.sleep_for(x)
    print(f"done {x}")

async def main():
    await asyncio.gather(*[
        thing(i)
        for i in range(0, 8)
    ])

asyncio.run(main())
