import sys
import subprocess
import json
import shutil
import os

assert __name__ == "__main__"

target = sys.argv[1]

cargo_build_output = subprocess.check_output(
    [
        "cargo",
        "build",
        "--target",
        target,
        "--release",
        "--bin",
        "editres",
        "--message-format=json",
    ],
    cwd=os.path.normpath(__file__ + "/../.."),
).splitlines()

executable_path = None

for line in cargo_build_output:
    line_json = json.loads(line)
    if line_json["reason"] == "compiler-artifact" and line_json["executable"] != None:
        executable_path = line_json["executable"]
        break


zip_name = "editres_cli_" + target

shutil.make_archive(
    zip_name,
    "zip",
    os.path.dirname(executable_path),
    os.path.basename(executable_path),
)

print(zip_name + ".zip")