if [ -x /opt/homebrew/bin/brew ]
  eval (/opt/homebrew/bin/brew shellenv)
end

set -x HOMEBREW_NO_ANALYTICS 1

if [ -d /home/linuxbrew/.linuxbrew ]
  set -x HOMEBREW_PREFIX "/home/linuxbrew/.linuxbrew"
  set -x HOMEBREW_CELLAR "/home/linuxbrew/.linuxbrew/Cellar"
  set -x HOMEBREW_REPOSITORY "/home/linuxbrew/.linuxbrew/Homebrew"
  set -x HOMEBREW_SHELLENV_PREFIX "/home/linuxbrew/.linuxbrew"
  set -x PATH "/home/linuxbrew/.linuxbrew/bin" "/home/linuxbrew/.linuxbrew/sbin" {$PATH}
  set -x MANPATH "/home/linuxbrew/.linuxbrew/share/man" {$MANPATH}
  set -x INFOPATH "/home/linuxbrew/.linuxbrew/share/info" {$INFOPATH}
end

if [ -d {$HOME}/.local/bin ]
  set -x PATH {$HOME}/.local/bin {$PATH}
end

if [ -d {$HOME}/.sdkman/candidates/kotlin/current/bin ]
  set -x PATH {$HOME}/.sdkman/candidates/kotlin/current/bin {$PATH}
end

if [ -d {$HOME}/.cargo/bin ]
  set -x PATH {$HOME}/.cargo/bin {$PATH}
end

if [ -d {$HOME}/.nimble/bin ]
  set -x PATH {$HOME}/.nimble/bin {$PATH}
end

if [ -d /usr/local/go/bin ]
  set -x PATH /usr/local/go/bin {$PATH}
end

set -x GOPATH {$HOME}/.go
if [ -n {$GOPATH} ]
  set -x PATH {$GOPATH}/bin {$PATH}
end

if [ -n {$HOME}/.volta ]
  set -x VOLTA_HOME {$HOME}/.volta
end
if [ -n {$HOME}/.volta/bin ]
  set -x PATH {$HOME}/.volta/bin {$PATH}
end

set -x PYENV_ROOT {$HOME}/.pyenv
if [ -n {$PYENV_ROOT}/bin ]
  set -x PATH {$PYENV_ROOT}/bin {$PATH}
end
if type -q pyenv
  pyenv init - | source
end

if [ -n {$HOME}/bin ]
  set -x PATH {$HOME}/bin {$PATH}
end

if ! [ -z {$SSH_CONNECTION} ]
  if type -q tmux
    if [ -z {$TMUX} ]
      if tmux list-sessions > /dev/null 2>&1
        exec tmux -u attach
      else
        exec tmux -u
      end
    end
  end
end

set -x XDG_CONFIG_HOME {$HOME}/.config
set -x XDG_CACHE_HOME {$HOME}/.cache

if type -q vim
  set -x EDITOR vim
  set -x VISUAL vim
else if type -q vi
  set -x EDITOR vi
  set -x VISUAL vi
else if type -q nvim
  set -x EDITOR nvim
  set -x VISUAL nvim
else
  echo 'vimもnvimもviもありませんでした'
end

alias less "less -cmN"

if type -q starship
  starship init fish | source
end

switch (uname)
  case Darwin
    alias safari "open -a /Applications/Safari.app"
    alias chrome "open -a /Applications/Google\ Chrome.app"
    alias firefox "open -a /Applications/Firefox.app"
    alias ls "ls -FG"
    alias la "ls -aFG"
    alias ll "ls -FGl"
  case Linux
    alias ls "ls -F --color=auto"
    alias la "ls -aF --color=auto"
    alias ll "ls -Fl --color=auto"
end

if uname -r | grep -i Microsoft > /dev/null 2>&1
  alias firefox "/mnt/c/Program\ Files/Mozilla\ Firefox/firefox.exe"
end

set -gx PATH "$HOME/.local/bin" $PATH

### MANAGED BY RANCHER DESKTOP START (DO NOT EDIT)
set --export --prepend PATH "$HOME/.rd/bin"
### MANAGED BY RANCHER DESKTOP END (DO NOT EDIT)
