import os
from ..utils import run_command


def update_sdkman():
    sdkman_init_script = os.path.expanduser("~/.sdkman/bin/sdkman-init.sh")
    if not os.path.isfile(sdkman_init_script):
        return

    command = f'source "{sdkman_init_script}" && sdk selfupdate && sdk update'
    run_command(
        command,
        "sdkman: self-update and update packages",
        shell=True,
        executable="/bin/bash",
    )


if __name__ == "__main__":
    update_sdkman()
