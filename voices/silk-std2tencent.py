from functools import reduce
from os import SEEK_SET, listdir, path
from multiprocessing import Pool


def detect_silk(file_path: str):
    if not path.isfile(file_path):
        return
    if not file_path.endswith(".silk"):
        return
    with open(file_path, "rb+") as f:
        data = f.read()

        if data.startswith(b"#!SILK_V3") and data.endswith(b"\xff\xff"):
            print(f"{file_path}: std voice, converting...")
            data = b"\x02" + data[:-2]
            f.seek(SEEK_SET)
            f.truncate()
            f.write(data)
        elif data.startswith(b"\x02#!SILK_V3"):
            print(f"{file_path}: tencent voice, skip")


def main():
    files = reduce(
        lambda a, b: a + b,
        [[path.join(d, f) for f in listdir(d)] for d in listdir(".") if path.isdir(d)],
    )
    with Pool() as pool:
        pool.map(detect_silk, files)


if __name__ == "__main__":
    main()
