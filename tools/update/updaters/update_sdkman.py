import subprocess
import os
from ..utils import print_command


def update_sdkman():
    errors = []
    sdkman_init_script = os.path.expanduser("~/.sdkman/bin/sdkman-init.sh")
    if not os.path.isfile(sdkman_init_script):
        return errors

    command = f'source "{sdkman_init_script}" && sdk selfupdate && sdk update'
    print_command(command)
    try:
        subprocess.run(command, shell=True, check=True, executable="/bin/bash")
    except subprocess.CalledProcessError:
        errors.append(command)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_sdkman()
