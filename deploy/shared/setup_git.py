
import os
import subprocess
from paths import HOME_DIR

def setup_git():
    """
    Gitの共通設定（macOSとAndroidで重複する設定）を行う。
    """
    print("--- Setting up common Git configurations ---")

    # ~/.gitconfigが存在しなかったら作成する
    # このファイルが存在しなかった場合、`git config --global`で設定したときに~/.config/git/configに書き込まれてしまう
    gitconfig_path = os.path.join(HOME_DIR, '.gitconfig')
    if not os.path.exists(gitconfig_path):
        print(f"  Creating empty {gitconfig_path}")
        open(gitconfig_path, 'a').close()

    # git diffしたときの文字コードをutf-8にする
    subprocess.run(["git", "config", "--global", "core.pager", "LESSCHARSET=utf-8 less -cmN"], check=True)
    print("  Set git config core.pager.")

    # プッシュするときは改行コードをLFに変換する
    # プルするときは変換しない
    subprocess.run(["git", "config", "--global", "core.autocrlf", "input"], check=True)
    print("  Set git config core.autocrlf.")

    print("--- Common Git setup complete ---\n")

if __name__ == '__main__':
    setup_git()
