import subprocess
import os


def update_sdkman():
    sdkman_init_script = os.path.expanduser("~/.sdkman/bin/sdkman-init.sh")
    if not os.path.isfile(sdkman_init_script):
        return

    print()
    print("####################")
    print("#                  #")
    print("#     `sdkman`     #")
    print("#                  #")
    print("####################")
    print()
    command = f'source "{sdkman_init_script}" && sdk selfupdate && sdk update'
    subprocess.run(command, shell=True, check=True, executable="/bin/bash")
    print()
    print()


if __name__ == "__main__":
    update_sdkman()
