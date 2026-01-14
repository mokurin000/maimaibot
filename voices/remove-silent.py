from pathlib import Path
from functools import reduce
from os import listdir, remove
from multiprocessing import Pool

from pydub import AudioSegment, silence


def remove_silent(file: Path):
    if not file.name.endswith("wav"):
        return
    audio: AudioSegment = AudioSegment.from_wav(file)

    silent_ranges = silence.detect_silence(audio, min_silence_len=1, silence_thresh=-40)
    silent_ranges_sec = [
        ((start / 1000), (stop / 1000)) for start, stop in silent_ranges
    ]

    if (
        silent_ranges_sec
        and abs(sum(silent_ranges_sec.pop()) - audio.duration_seconds) < 0.01
    ):
        return True
    return False


def process(file: Path):
    if remove_silent(file):
        remove(file)


def main():
    dirs = [
        Path("dump").joinpath(d)
        for d in listdir("dump")
        if d.startswith("Voice_") or d.startswith("Mai2")
    ]
    files = reduce(
        lambda a, b: a + b,
        map(
            lambda d: list(
                map(
                    lambda f: Path(d).joinpath(f).absolute(),
                    listdir(d),
                )
            ),
            dirs,
        ),
    )

    with Pool() as pool:
        pool.map(process, files)


if __name__ == "__main__":
    main()
