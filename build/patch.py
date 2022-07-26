#!/usr/bin/env python
# pylint: disable=all

import sys
import re


def main():
    content = open(sys.argv[1]).read()
    m = re.search("(export{(.*)};)", content)
    if not m:
        return
    [whole, exports] = m.groups(0)

    exports = exports.split(",")
    reexports = []

    for export in exports:
        m2 = re.search("([^ ]+) as ([^ ]+)", export)
        if not m2:
            continue

        [key, export_name] = m2.groups(0)
        code = f"""{export_name}: {key}"""
        reexports.append(code)

    exports = ", ".join(reexports)
    new_whole = f"; window.monacoPackage = {{{exports}}};"
    content = content.replace(whole, new_whole)
    print(sys.argv[1], "patched")
    open(sys.argv[1], "w").write(content)


if __name__ == "__main__":
    main()
