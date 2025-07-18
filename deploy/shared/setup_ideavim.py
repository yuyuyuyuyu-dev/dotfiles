import os
from deploy.shared.paths import DOTFILES_ROOT, HOME_DIR
from deploy.shared.create_symlink_safely import create_symlink_safely


def setup_ideavim():
    """
    IdeaVimの設定ファイルをセットアップする。
    .ideavimrc のシンボリックリンクをホームディレクトリに作成する。
    """
    print("--- Setting up IdeaVim configuration ---")

    source_path = os.path.join(DOTFILES_ROOT, ".ideavimrc")
    dest_path = os.path.join(HOME_DIR, ".ideavimrc")

    create_symlink_safely(source_path, dest_path)

    print("--- IdeaVim setup complete ---\n")


if __name__ == "__main__":
    setup_ideavim()
