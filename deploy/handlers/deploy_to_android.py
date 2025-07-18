#!/usr/bin/env python3

import os
import subprocess
from deploy.shared.setup_bash import setup_bash
from deploy.shared.setup_git import setup_git
from deploy.shared.setup_vim import setup_vim
from deploy.shared.paths import HOME_DIR


def deploy_to_android():
    """Android用のデプロイを実行する"""
    print("Starting deployment for Android...")

    setup_bash()
    setup_git()

    _setup_git_for_platform()

    setup_vim()

    print("\nAndroid deployment finished.")


def _setup_git_for_platform():
    """
    Android固有のGit設定を行う。
    """
    print("--- Setting up platform specific Git configurations (Android) ---")

    # デフォルトブランチの名前を"main"に指定する
    subprocess.run(
        ["git", "config", "--global", "init.defaultBranch", "main"], check=True
    )
    print("  Set git config init.defaultBranch.")

    # プッシュするときの認証に使う公開鍵を指定する
    subprocess.run(
        [
            "git",
            "config",
            "--global",
            "user.signingkey",
            os.path.join(HOME_DIR, ".ssh/id_ed25519.pub"),
        ],
        check=True,
    )
    print("  Set git config user.signingkey.")

    # git diffしたときに、改行コードを気にしないようにする
    subprocess.run(["git", "config", "--global", "alias.diff", "diff -w"], check=True)
    print("  Set git config alias.diff.")

    # 常に読み込まれるgitignoreファイルを指定する
    subprocess.run(
        [
            "git",
            "config",
            "--global",
            "core.excludesFile",
            os.path.join(HOME_DIR, ".config/git/ignore"),
        ],
        check=True,
    )
    print("  Set git config core.excludesFile.")

    # `git root` でリポジトリのルートディレクトリを出力する
    subprocess.run(
        ["git", "config", "--global", "alias.root", "rev-parse --show-toplevel"],
        check=True,
    )
    print("  Set git config alias.root.")

    # プルしたときの設定
    subprocess.run(["git", "config", "--global", "pull.ff", "true"], check=True)
    print("  Set git config pull.ff.")

    # コミットに署名をする
    subprocess.run(["git", "config", "--global", "commit.gpgsign", "true"], check=True)
    print("  Set git config commit.gpgsign.")
    subprocess.run(["git", "config", "--global", "gpg.format", "ssh"], check=True)
    print("  Set git config gpg.format.")

    subprocess.run(["git", "config", "--global", "alias.pull", "pull -p"], check=True)
    print("  Set git config alias.pull.")
    subprocess.run(["git", "config", "--global", "alias.fetch", "fetch -p"], check=True)
    print("  Set git config alias.fetch.")

    print("--- Platform specific Git setup complete (Android) ---\n")
