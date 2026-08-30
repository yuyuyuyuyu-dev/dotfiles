if type nvim > /dev/null 2>&1 ; then
  export EDITOR='nvim'
  export VISUAL='nvim'
elif type vim > /dev/null 2>&1 ; then
  export EDITOR='vim'
  export VISUAL='vim'
elif type vi > /dev/null 2>&1 ; then
  export EDITOR='vi'
  export VISUAL='vi'
else
  echo 'nvimもvimもviもありませんでした'
fi

export XDG_CONFIG_HOME="${HOME}/.config"
export XDG_CACHE_HOME="${HOME}/.cache"

export VOLTA_HOME="$HOME/.volta"
export PATH="$VOLTA_HOME/bin:$PATH"

export JAVA_HOME="${HOME}/.sdkman/candidates/java/current"

export ANDROID_HOME=~/Library/Android/sdk
export PATH="$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools:$PATH"

if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi

export PATH=$HOME/.local/bin:$PATH

export PATH=/opt/homebrew/opt/qt@5/bin:$PATH

case "$OSTYPE" in
  darwin*)
    ;;
  linux*)
    ;;
esac

local current_dir="${${(%):-%x}:A:h}"
local dotfiles_private_zshenv_path="${current_dir}/../dotfiles-private/.zshenv"
if [ -f "$dotfiles_private_zshenv_path" ]; then
  source "$dotfiles_private_zshenv_path"
fi
