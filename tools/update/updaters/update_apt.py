import subprocess
import shutil


def update_apt():
    if not shutil.which("apt-get"):
        return

    print()
    print()
    print("########")
    print("apt")
    print("########")
    subprocess.run(["sudo", "apt", "update"], check=True)
    subprocess.run(["sudo", "apt", "-y", "upgrade"], check=True)


if __name__ == "__main__":
    update_apt()
