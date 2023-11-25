with open("./source.ty", "r") as f:
    src = f.read()

HIRAGANA = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをんぁぃぅぇぉっゃょ" + "がぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ"

for line in src.split("\n"):
    segments = line.split("\t")

    if len(segments) != 3:
        break

    origin = list(filter(lambda d: d != " ", list(segments[1])))
    hira = list(segments[2])

    origin_segments = []
    hira_segments = []
    ok = True

    while True:
        origin_buffer = []
        hira_buffer = []

        while len(origin) > 0 and origin[0] in HIRAGANA:
            origin_char = origin.pop(0)
            hira_char = hira.pop(0)

            if origin_char != hira_char:
                ok = False
                print("// sentence auto split failed; manual operation required")
                print(f"// {segments[1]}")
                print(f"// {segments[2]}")
                print(f"// origin: {origin_buffer} {origin_char}")
                print(f"// hira:   {hira_buffer} {hira_char}")
                print()
                break

            origin_buffer.append(origin_char)
            hira_buffer.append(hira_char)

        if not ok:
            break

        if len(origin_buffer) != 0:
            origin_segments.append("".join(origin_buffer))
            hira_segments.append("".join(hira_buffer))
        if len(origin) == 0:
            break

        origin_buffer = []
        hira_buffer = []

        while len(origin) != 0 and origin[0] not in HIRAGANA:
            origin_buffer.append(origin.pop(0))

        if len(origin) == 0:
            hira_buffer = hira
        else:
            while len(hira_buffer) == 0 or origin[0] != hira[0]:
                hira_buffer.append(hira.pop(0))

        if len(origin_buffer) != 0:
            o = "".join(origin_buffer)
            h = "".join(hira_buffer)
            origin_segments.append(o)
            hira_segments.append(h)

    if ok:
        print("&segments![")
        print("    \"" + "\" / \"".join(origin_segments) + "\",")
        print("    \"" + "\" / \"".join(hira_segments) + "\",")
        print("],")
