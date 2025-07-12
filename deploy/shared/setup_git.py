
import os
import subprocess

def setup_git():
    """
    Gitの共通設定（macOSとAndroidで重複する設定）を行う。
    """
    print("--- Setting up common Git configurations ---")

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

