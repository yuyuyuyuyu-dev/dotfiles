#!/usr/bin/env python3

from deploy.shared.setup_bash import setup_bash
from deploy.shared.setup_zsh import setup_zsh
from deploy.shared.setup_git import setup_git
from deploy.shared.setup_tmux import setup_tmux
from deploy.shared.deploy_configs import deploy_configs
from deploy.shared.setup_vim import setup_vim
from deploy.shared.setup_ideavim import setup_ideavim
from deploy.shared.setup_update_command import setup_update_command


def deploy_to_mac():
    """macOS用のデプロイを実行する"""
    print("Starting deployment for macOS...")

    setup_bash()
    setup_zsh()

    setup_git()

    setup_tmux()

    setup_vim()
    setup_ideavim()
    setup_update_command()

    deploy_configs()

    print("\nmacOS deployment finished.")
