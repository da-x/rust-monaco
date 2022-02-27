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

    export_funcs = []
    exports = exports.split(",")

    for export in exports:
        m2 = re.search("([^ ]+) as ([^ ]+)", export)
        if not m2:
            continue

        [key, export_name] = m2.groups(0)
        export_func_name = f"_export_get_{export_name}"
        if export_name != export_name.lower():
            continue
        code = f"""function {export_func_name}() {{
            return {key}
        }}"""
        export_funcs.append(code)
        exports.append(export_func_name)

    exports = ",".join(exports)
    prev_whole = whole
    code = ";".join(export_funcs).replace("\n", ";")
    whole = f"{code};export{{{exports}}}"
    content = content.replace(prev_whole, whole)
    print(sys.argv[1], "patched")
    open(sys.argv[1], "w").write(content)


if __name__ == "__main__":
    main()
