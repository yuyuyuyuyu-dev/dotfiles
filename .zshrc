 PROMPT='[%~] 🍓 %(?.%F{#bbdefb}.%F{#dc143c})❯%f '

alias less="less -cmN"
alias gemini='gemini -m gemini-2.5-flash'

setopt auto_cd

setopt nolistbeep

setopt auto_pushd

setopt pushd_ignore_dups

setopt auto_param_slash

setopt list_types

setopt mark_dirs

setopt hist_ignore_dups

zstyle ':completion:*:default' menu select=1

zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
zstyle ':completion:*' completer _expand _complete _match _prefix _approximate _list _history

autoload -U compinit; compinit

bindkey -e

zmodload zsh/complist
bindkey -M menuselect 'h' vi-backward-char
bindkey -M menuselect 'j' vi-down-line-or-history
bindkey -M menuselect 'k' vi-up-line-or-history
bindkey -M menuselect 'l' vi-forward-char

WORDCHARS='*?_-.[]~=&;!#$%^(){}<>'

case "${OSTYPE}" in
  darwin*)
    alias safari="open -a /Applications/Safari.app"
    alias chrome="open -a /Applications/Google\ Chrome.app"
    alias firefox="open -a /Applications/Firefox.app"
    alias ls="ls -FG"
    alias la="ls -aFG"
    alias ll="ls -FGl"
    ;;
  linux*)
    alias ls="ls -F --color=auto"
    alias la="ls -aF --color=auto"
    alias ll="ls -Fl --color=auto"
    ;;
esac

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"

source <(ng completion script)

export PATH="$HOME/.antigravity/antigravity/bin:$PATH"

export PATH="$HOME/.antigravity-ide/antigravity-ide/bin:$PATH"

export PATH="$HOME/.local/bin:$PATH"

# pnpm
export PNPM_HOME="$HOME/Library/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME/bin:"*) ;;
  *) export PATH="$PNPM_HOME/bin:$PATH" ;;
esac
# pnpm end

### MANAGED BY RANCHER DESKTOP START (DO NOT EDIT)
export PATH="$HOME/.rd/bin:$PATH"
### MANAGED BY RANCHER DESKTOP END (DO NOT EDIT)
