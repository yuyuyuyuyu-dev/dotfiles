from deploy.shared.deploy_configs import deploy_configs
from deploy.shared.setup_bash import setup_bash
from deploy.shared.setup_claude import setup_claude
from deploy.shared.setup_claude_gh_admission_hook import setup_claude_gh_admission_hook
from deploy.shared.setup_gemini import setup_gemini
from deploy.shared.setup_git import setup_git
from deploy.shared.setup_ideavim import setup_ideavim
from deploy.shared.setup_tmux import setup_tmux
from deploy.shared.setup_vim import setup_vim
from deploy.shared.setup_zsh import setup_zsh


def deploy_to_mac():
    print("Starting deployment for macOS...")

    setup_bash()
    setup_zsh()

    setup_git()

    setup_tmux()

    setup_vim()
    setup_ideavim()
    setup_claude()
    setup_gemini()

    deploy_configs()

    setup_claude_gh_admission_hook()

    print("\nmacOS deployment finished.")
