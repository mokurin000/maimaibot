from functools import reduce
from os import listdir, path, remove
from multiprocessing import Pool


def remove_wav_pcm(file: str):
    if not path.isfile(file):
        return
    if not file.endswith(".wav") and not file.endswith(".pcm"):
        return
    remove(file)


def main():
    files = reduce(
        lambda a, b: a + b,
        [[path.join(d, f) for f in listdir(d)] for d in listdir(".") if path.isdir(d)],
    )
    with Pool() as pool:
        pool.map(remove_wav_pcm, files)


if __name__ == "__main__":
    main()
