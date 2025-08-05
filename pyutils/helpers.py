from decimal import Decimal, getcontext

import orjson as json

getcontext().prec = 28


def dx_rating(difficulty: Decimal, achievement: int) -> int:
    # Constants
    SSS_PLUS_THRESHOLD = Decimal("100.5")
    SSS_PLUS_FACTOR = Decimal("0.224")
    SSS_PRO_THRESHOLD = Decimal("100.4999")
    SSS_PRO_FACTOR = Decimal("0.222")
    SSS_THRESHOLD = Decimal("100.0")
    SSS_FACTOR = Decimal("0.216")
    SS_PLUS_PRO_THRESHOLD = Decimal("99.9999")
    SS_PLUS_PRO_FACTOR = Decimal("0.214")
    SS_PLUS_THRESHOLD = Decimal("99.5")
    SS_PLUS_FACTOR = Decimal("0.211")
    SS_THRESHOLD = Decimal("99.0")
    SS_FACTOR = Decimal("0.208")
    S_PLUS_PRO_THRESHOLD = Decimal("98.9999")
    S_PLUS_PRO_FACTOR = Decimal("0.206")
    S_PLUS_THRESHOLD = Decimal("98.0")
    S_PLUS_FACTOR = Decimal("0.203")
    S_THRESHOLD = Decimal("97.0")
    S_FACTOR = Decimal("0.2")
    AAA_PRO_THRESHOLD = Decimal("96.9999")
    AAA_PRO_FACTOR = Decimal("0.176")
    AAA_THRESHOLD = Decimal("94.0")
    AAA_FACTOR = Decimal("0.168")
    AA_THRESHOLD = Decimal("90.0")
    AA_FACTOR = Decimal("0.152")
    A_THRESHOLD = Decimal("80.0")
    A_FACTOR = Decimal("0.136")

    ach = Decimal(achievement) / Decimal("10000")
    if ach > Decimal("101.0") or ach < A_THRESHOLD:
        return 0
    if ach >= SSS_PLUS_THRESHOLD:
        factor = SSS_PLUS_FACTOR
        ach = Decimal("100.5")
    elif ach >= SSS_PRO_THRESHOLD:
        factor = SSS_PRO_FACTOR
    elif ach >= SSS_THRESHOLD:
        factor = SSS_FACTOR
    elif ach >= SS_PLUS_PRO_THRESHOLD:
        factor = SS_PLUS_PRO_FACTOR
    elif ach >= SS_PLUS_THRESHOLD:
        factor = SS_PLUS_FACTOR
    elif ach >= SS_THRESHOLD:
        factor = SS_FACTOR
    elif ach >= S_PLUS_PRO_THRESHOLD:
        factor = S_PLUS_PRO_FACTOR
    elif ach >= S_PLUS_THRESHOLD:
        factor = S_PLUS_FACTOR
    elif ach >= S_THRESHOLD:
        factor = S_FACTOR
    elif ach >= AAA_PRO_THRESHOLD:
        factor = AAA_PRO_FACTOR
    elif ach >= AAA_THRESHOLD:
        factor = AAA_FACTOR
    elif ach >= AA_THRESHOLD:
        factor = AA_FACTOR
    elif ach >= A_THRESHOLD:
        factor = A_FACTOR
    else:
        return 0
    result = (factor * difficulty * ach).quantize(Decimal("1."), rounding="ROUND_FLOOR")
    return int(result)


with open("musicDB.json", "r", encoding="utf-8") as f:
    MUSIC_DB = json.loads(f.read())

MUSIC_DB = {entry["id"]: entry for entry in MUSIC_DB}


def query_music_db(music_id: int):
    music_info = MUSIC_DB.get(music_id)
    if music_info is None:
        return
    return music_info


def find_level(music_info: dict, level_id: int):
    return [level for level in music_info["levels"] if level["level"] == level_id]
