import subprocess
import shutil


def update_rust():
    if shutil.which("rustup"):
        print()
        print("####################")
        print("#                  #")
        print("#     `rustup`     #")
        print("#                  #")
        print("####################")
        print()
        subprocess.run(["rustup", "update"], check=True)
        print()
        print()

    if shutil.which("cargo"):
        try:
            subprocess.run(
                ["cargo", "install-update", "--version"],
                check=True,
                capture_output=True,
            )
            print()
            print("####################")
            print("#                  #")
            print("#      `cargo`     #")
            print("#                  #")
            print("####################")
            print()
            subprocess.run(["cargo", "install-update", "-a"], check=True)
            print()
            print()
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass


if __name__ == "__main__":
    update_rust()
