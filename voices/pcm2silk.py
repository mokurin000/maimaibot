import subprocess
from functools import reduce
from os import cpu_count, listdir, path
from multiprocessing import Pool

rate = "24000"


def process_pcm(file_path: str):
    if not path.isfile(file_path):
        return
    if not file_path.endswith(".pcm"):
        return
    output = file_path.replace(".pcm", ".silk")

    subprocess.run(
        ["./silk-encoder.exe", file_path, output, "-Fs_API", rate, "-rate", rate]
    )


def main():
    files = reduce(
        lambda a, b: a + b,
        [[path.join(d, f) for f in listdir(d)] for d in listdir(".") if path.isdir(d)],
    )
    with Pool(processes=cpu_count() * 4) as pool:
        pool.map(process_pcm, files)


if __name__ == "__main__":
    main()
