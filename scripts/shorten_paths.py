import os
import re
from pathlib import Path

USE_PATTERN = re.compile(r'^\s*(?:pub\s+)?use\s+([a-zA-Z0-9_:]+)::.*;', re.MULTILINE)


def process_file(path: str) -> None:
    """Rewrite fully qualified paths only after their imports."""
    text = Path(path).read_text()
    lines = text.splitlines(keepends=True)

    imports = []
    for idx, line in enumerate(lines):
        m = USE_PATTERN.match(line)
        if m:
            imports.append(m.group(1))
            continue  # do not modify import lines

        for imp in sorted(imports, key=len, reverse=True):
            qualified = f"{imp}::"
            if qualified in line:
                line = line.replace(qualified, "")
        lines[idx] = line

    new_text = "".join(lines)
    if new_text != text:
        Path(path).write_text(new_text)
        print(f"Updated: {path}")


def main():
    for root, _, files in os.walk('.'):
        for f in files:
            if f.endswith('.rs'):
                process_file(os.path.join(root, f))


if __name__ == '__main__':
    main()
