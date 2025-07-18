import os
from deploy.shared.paths import DOTFILES_ROOT, HOME_DIR
from deploy.shared.create_symlink_safely import create_symlink_safely


def setup_tmux():
    """
    tmuxの設定ファイルをセットアップする。
    .tmux.conf のシンボリックリンクをホームディレクトリに作成する。
    """
    print("--- Setting up tmux configuration ---")

    source_path = os.path.join(DOTFILES_ROOT, ".tmux.conf")
    dest_path = os.path.join(HOME_DIR, ".tmux.conf")

    create_symlink_safely(source_path, dest_path)

    print("--- tmux setup complete ---\n")


if __name__ == "__main__":
    setup_tmux()
