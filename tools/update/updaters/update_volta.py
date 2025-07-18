import subprocess
import shutil


def update_volta():
    if not shutil.which("volta"):
        return

    print()
    print()
    print("########")
    print("volta")
    print("########")
    subprocess.run("curl https://get.volta.sh | bash", shell=True, check=True)
    subprocess.run(["volta", "install", "node"], check=True)
    subprocess.run(["volta", "install", "npm"], check=True)


if __name__ == "__main__":
    update_volta()
