import os
from . import paths
from .create_symlink_safely import create_symlink_safely


def setup_update_command():
    """
    `update` コマンドのシンボリックリンクを `~/.local/bin` に作成する。
    """
    src = os.path.join(paths.DOTFILES_ROOT, "bin", "update")
    dest = os.path.join(paths.LOCAL_BIN_DIR, "update")

    os.makedirs(os.path.dirname(dest), exist_ok=True)
    create_symlink_safely(src, dest)


if __name__ == "__main__":
    setup_update_command()
