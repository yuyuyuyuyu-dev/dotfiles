import subprocess
import os
from ..utils import print_command


def update_sdkman():
    sdkman_init_script = os.path.expanduser("~/.sdkman/bin/sdkman-init.sh")
    if not os.path.isfile(sdkman_init_script):
        return

    command = f'source "{sdkman_init_script}" && sdk selfupdate && sdk update'
    print_command(command)
    subprocess.run(command, shell=True, check=True, executable="/bin/bash")
    print()
    print()


if __name__ == "__main__":
    update_sdkman()
