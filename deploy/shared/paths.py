import os

_current_file_path = os.path.abspath(__file__)

_shared_dir = os.path.dirname(_current_file_path)

_deploy_dir = os.path.dirname(_shared_dir)

DOTFILES_ROOT = os.path.dirname(_deploy_dir)

HOME_DIR = os.path.expanduser("~")

if __name__ == "__main__":
    print(f"DOTFILES_ROOT: {DOTFILES_ROOT}")
    print(f"HOME_DIR: {HOME_DIR}")
