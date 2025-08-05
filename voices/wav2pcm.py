import subprocess
from functools import reduce
from os import listdir, path
from multiprocessing import Pool


def process_wav(file: str):
    if not path.isfile(file):
        return
    if not file.endswith(".wav"):
        return
    output = file.replace(".wav", ".pcm")

    subprocess.run(
        ["ffmpeg", "-y", "-i", file, "-ar", "24000", "-ac", "1", "-f", "s16le", output]
    )


def main():
    files = reduce(
        lambda a, b: a + b,
        [[path.join(d, f) for f in listdir(d)] for d in listdir(".") if path.isdir(d)],
    )
    with Pool() as pool:
        pool.map(process_wav, files)


if __name__ == "__main__":
    main()
