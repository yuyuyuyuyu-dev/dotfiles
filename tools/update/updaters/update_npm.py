import subprocess
import shutil


def update_npm():
    if not shutil.which("npm"):
        return

    print()
    print()
    print("########")
    print("npm modules")
    print("########")
    subprocess.run(["npm", "outdated", "-g", "--depth=0"])
    subprocess.run(["npm", "update", "-g"], check=True)


if __name__ == "__main__":
    update_npm()
