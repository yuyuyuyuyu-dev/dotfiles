#!/usr/bin/env python3

from deploy.shared.setup_bash import setup_bash
from deploy.shared.setup_zsh import setup_zsh
from deploy.shared.setup_git import setup_git
from deploy.shared.deploy_configs import deploy_configs
from deploy.shared.setup_vim import setup_vim
from deploy.shared.setup_update_command import setup_update_command


def deploy_to_linux():
    """Linux用のデプロイを実行する"""
    print("Starting deployment for Linux...")

    setup_bash()
    setup_zsh()

    setup_git()

    setup_vim()
    setup_update_command()

    deploy_configs()

    print("\nLinux deployment finished.")
