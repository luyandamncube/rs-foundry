import os
import sys
import re


def strip_comment(line: str) -> str:
    """
    Remove inline comments from a tree line.
    Only strips comments that start with whitespace + #,
    so names containing '#' are less likely to break.
    """
    return re.sub(r"\s+#.*$", "", line).rstrip()


def get_level(line: str) -> int:
    """
    Determine nesting level from tree format.
    Count occurrences of either '│   ' or '    ' groups.
    """
    prefix_match = re.match(r"^((?:│   |    )*)", line)
    if not prefix_match:
        return 0

    prefix = prefix_match.group(1)
    return len(prefix) // 4


def extract_name(line: str) -> str:
    """
    Remove tree drawing characters and return actual file/folder name.
    """
    line = re.sub(r"^((?:│   |    )*)?(├── |└── )", "", line)
    return line.strip()


def scaffold_from_tree(tree_text: str):
    stack = []

    for raw_line in tree_text.splitlines():
        line = strip_comment(raw_line)

        if not line.strip():
            continue

        # Root folder (no ├── or └──)
        if "├──" not in line and "└──" not in line:
            name = line.strip()
            stack = [name.rstrip("/")]
            os.makedirs(stack[0], exist_ok=True)
            print(f"Created dir: {stack[0]}")
            continue

        level = get_level(line)
        name = extract_name(line)

        is_dir = name.endswith("/")
        clean_name = name.rstrip("/")

        # Adjust stack depth (root is stack[0], children follow)
        stack = stack[: level + 1]
        stack.append(clean_name)

        path = os.path.join(*stack)

        if is_dir:
            os.makedirs(path, exist_ok=True)
            print(f"Created dir: {path}")
        else:
            parent = os.path.dirname(path)
            if parent:
                os.makedirs(parent, exist_ok=True)
            open(path, "a").close()
            print(f"Created file: {path}")

    print("\n✅ Scaffold created correctly.")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python scaffold_from_tree.py DIRTREE.txt")
        sys.exit(1)

    with open(sys.argv[1], "r", encoding="utf-8") as f:
        tree = f.read()

    scaffold_from_tree(tree)