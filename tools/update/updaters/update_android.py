import subprocess
import shutil
from ..utils import print_command


def update_android():
    errors = []
    if not shutil.which("android"):
        return errors

    cmd = "android update"
    print_command(cmd)
    try:
        subprocess.run(["android", "update"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd)
    print()
    print()

    return errors
