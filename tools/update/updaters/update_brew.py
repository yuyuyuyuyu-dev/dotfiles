import subprocess
import shutil


def update_brew():
    if not shutil.which("brew"):
        return

    print()
    print("####################")
    print("#                  #")
    print("#      `brew`      #")
    print("#                  #")
    print("####################")
    print()
    subprocess.run(["brew", "update", "-v"], check=True)
    subprocess.run(["brew", "upgrade", "-v"], check=True)
    print()
    print()


if __name__ == "__main__":
    update_brew()
