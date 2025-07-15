import subprocess
import shutil


def update_python():
    if shutil.which("pyenv"):
        try:
            subprocess.run(
                ["pyenv", "update", "--help"], check=True, capture_output=True
            )
            print()
            print()
            print("########")
            print("pyenv")
            print("########")
            subprocess.run(["pyenv", "update"], check=True)
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    if shutil.which("pip3"):
        print()
        print()
        print("########")
        print("pip")
        print("########")
        subprocess.run(["pip3", "install", "-U", "pip"], check=True)


if __name__ == "__main__":
    update_python()
