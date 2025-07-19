# どんな時でも必ず最初に読み込まれるファイル
# zshを使う時は必ず必要な設定はここに書く

# -- 環境変数 --
# デフォルトのエディタの設定
# なぜかzcompileされるとnvimが見つからない
if type nvim > /dev/null 2>&1 ; then
  # nvimがあったらnvimに設定
  export EDITOR='nvim'
  export VISUAL='nvim'
  # echo 'nvimに設定'
elif type vim > /dev/null 2>&1 ; then
  # nvimがなくてvimがあればvimに設定
  export EDITOR='vim'
  export VISUAL='vim'
  # echo 'vimに設定'
elif type vi > /dev/null 2>&1 ; then
  # nvimもvimもなくてviがあればviに設定
  export EDITOR='vi'
  export VISUAL='vi'
  # echo 'viに設定'
else
  echo 'nvimもvimもviもありませんでした'
fi

# neovimに必要な設定
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

case "$OSTYPE" in
  darwin*)
    # darwin(≒Mac)のときに読み込まれる設定
    ;;
  linux*)
    # linuxのときに読み込まれる設定
    ;;
esac
