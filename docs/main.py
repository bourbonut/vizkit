content = """
    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#440154" height="30" width="30" x="0"/>
    /// <rect fill="#482475" height="30" width="30" x="30"/>
    /// <rect fill="#414487" height="30" width="30" x="60"/>
    /// <rect fill="#355f8d" height="30" width="30" x="90"/>
    /// <rect fill="#2a788e" height="30" width="30" x="120"/>
    /// <rect fill="#21918c" height="30" width="30" x="150"/>
    /// <rect fill="#22a884" height="30" width="30" x="180"/>
    /// <rect fill="#44bf70" height="30" width="30" x="210"/>
    /// <rect fill="#7ad151" height="30" width="30" x="240"/>
    /// <rect fill="#bddf26" height="30" width="30" x="270"/>
    /// <rect fill="#fde725" height="30" width="30" x="300"/>
    /// </svg>
    Viridis,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#000004" height="30" width="30" x="0"/>
    /// <rect fill="#140e36" height="30" width="30" x="30"/>
    /// <rect fill="#3b0f70" height="30" width="30" x="60"/>
    /// <rect fill="#641a80" height="30" width="30" x="90"/>
    /// <rect fill="#8c2981" height="30" width="30" x="120"/>
    /// <rect fill="#b73779" height="30" width="30" x="150"/>
    /// <rect fill="#de4968" height="30" width="30" x="180"/>
    /// <rect fill="#f7705c" height="30" width="30" x="210"/>
    /// <rect fill="#fe9f6d" height="30" width="30" x="240"/>
    /// <rect fill="#fecf92" height="30" width="30" x="270"/>
    /// <rect fill="#fcfdbf" height="30" width="30" x="300"/>
    /// </svg>
    Magma,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#000004" height="30" width="30" x="0"/>
    /// <rect fill="#160b39" height="30" width="30" x="30"/>
    /// <rect fill="#420a68" height="30" width="30" x="60"/>
    /// <rect fill="#6a176e" height="30" width="30" x="90"/>
    /// <rect fill="#932667" height="30" width="30" x="120"/>
    /// <rect fill="#bc3754" height="30" width="30" x="150"/>
    /// <rect fill="#dd513a" height="30" width="30" x="180"/>
    /// <rect fill="#f37819" height="30" width="30" x="210"/>
    /// <rect fill="#fca50a" height="30" width="30" x="240"/>
    /// <rect fill="#f6d746" height="30" width="30" x="270"/>
    /// <rect fill="#fcffa4" height="30" width="30" x="300"/>
    /// </svg>
    Inferno,

    /// <svg height="30" viewBox="0 0 300 30" width="300" xmlns="http://www.w3.org/2000/svg">
    /// <rect fill="#0d0887" height="30" width="30" x="0"/>
    /// <rect fill="#41049d" height="30" width="30" x="30"/>
    /// <rect fill="#6a00a8" height="30" width="30" x="60"/>
    /// <rect fill="#8f0da4" height="30" width="30" x="90"/>
    /// <rect fill="#b12a90" height="30" width="30" x="120"/>
    /// <rect fill="#cc4778" height="30" width="30" x="150"/>
    /// <rect fill="#e16462" height="30" width="30" x="180"/>
    /// <rect fill="#f2844b" height="30" width="30" x="210"/>
    /// <rect fill="#fca636" height="30" width="30" x="240"/>
    /// <rect fill="#fcce25" height="30" width="30" x="270"/>
    /// <rect fill="#f0f921" height="30" width="30" x="300"/>
    /// </svg>
    Plasma,
"""

svg = []
for line in content.split("\n"):
    line = line.strip()
    if line.startswith("///"):
        svg.append(line.removeprefix("/// "))
    elif len(line) != 0:
        name = line.lower().removesuffix(",")
        with open(f"{name}.svg", "w") as file:
            file.write("\n".join(svg))
        svg = []
        print(f'#[doc = include_str!("../../docs/{name}.svg")]')
        print(line)
        print("")
    else:
        continue
