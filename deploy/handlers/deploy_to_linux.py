from deploy.shared.deploy_configs import deploy_configs
from deploy.shared.setup_bash import setup_bash
from deploy.shared.setup_git import setup_git
from deploy.shared.setup_vim import setup_vim
from deploy.shared.setup_write_gate_for_github import setup_write_gate_for_github
from deploy.shared.setup_zsh import setup_zsh


def deploy_to_linux():
    print("Starting deployment for Linux...")

    setup_bash()
    setup_zsh()

    setup_git()

    setup_vim()

    deploy_configs()

    setup_write_gate_for_github()

    print("\nLinux deployment finished.")
