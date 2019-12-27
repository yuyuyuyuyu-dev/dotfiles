# ログインシェルかインタラクティブシェルのとき読み込まれるファイル
# シェルスクリプト実行時には不要な設定はここに書く

# プロンプトの設定
PROMPT=$'\n%m: %n$ '
RPROMPT=$'%~'

# コマンド履歴
HISTFILE=~/Library/Mobile\ Documents/com\~apple\~CloudDocs/.zsh_history  # 履歴の保存先
HISTSIZE=10000                                                           # メモリに保存される履歴の件数
SAVEHIST=1000000                                                         # 履歴ファイルに保存される履歴の件数

# パスの設定
path=(
  $path
  ~/MyCommands(N-/)
)

export LANG=ja_JP.UTF-8
# export LC_CTYPE=ja_JP.UTF-8

# エイリアスの設定
alias less="less -N"
alias wakeValet="wakeonlan 08:62:66:7c:e6:ba"
alias lsless="ls -CF | less -N"
alias del="delete"

# setopt
#  入力したコマンドが存在せず、かつディレクトリ名と一致するなら、ディレクトリに cd する
setopt auto_cd

# ビープ音を無効化
setopt nolistbeep

# cd した先のディレクトリをディレクトリスタックに追加する
# ディレクトリスタックとは今までに行ったディレクトリの履歴のこと
# `cd +<Tab>` でディレクトリの履歴が表示され、そこに移動できる
setopt auto_pushd

# pushd したとき、ディレクトリがすでにスタックに含まれていればスタックに追加しない
setopt pushd_ignore_dups

# ディレクトリ名の補完で末尾の / を自動的に付加し、次の補完に備える
setopt auto_param_slash

# 補完候補一覧でファイルの種別を識別マーク表示 (訳注:ls -F の記号)
setopt list_types

# ファイル名の展開でディレクトリにマッチした場合 末尾に / を付加
setopt mark_dirs

# 重複を記録しない
setopt hist_ignore_dups

# zstyle
# <Tab> でパス名の補完候補を表示したあと、
# 続けて <Tab> を押すと候補からパス名を選択できるようになる
# 候補を選ぶには <Tab> か Ctrl-N,B,F,P
zstyle ':completion:*:default' menu select=1

zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'  # Tab補完するときに大文字と小文字を区別しない
zstyle ':completion:*' completer _expand _complete _match _prefix _approximate _list _history  # いいかんじに補完してくれる

# autoload
autoload -U compinit; compinit # 自動補完を有効にする

# bindkey
bindkey -e   # emacsライクな操作を有効にする
# bindkey -v # viライクな操作を有効にする

# 補完候補のメニュー選択で、矢印キーの代わりにhjklで移動出来るようにする。
zmodload zsh/complist
bindkey -M menuselect 'h' vi-backward-char
bindkey -M menuselect 'j' vi-down-line-or-history
bindkey -M menuselect 'k' vi-up-line-or-history
bindkey -M menuselect 'l' vi-forward-char

# 単語の一部として扱われる文字のセットを指定する
# ここではデフォルトのセットから / を抜いたものとする
# こうすると、 Ctrl-w でカーソル前の1単語を削除したとき、 / までで削除が止まる
WORDCHARS='*?_-.[]~=&;!#$%^(){}<>'

case "${OSTYPE}" in
  darwin*)
    # darwin(≒Mac)のときに読み込まれる設定
    export HOMEBREW_GITHUB_API_TOKEN=2ece4e396f6394176eedec0c23682ad1411b5760
    export HOMEBREW_CASK_OPTS="--appdir=/Applications"

    # エイリアスの設定
    alias safari="open -a /Applications/Safari.app"
    alias chrome="open -a /Applications/Google\ Chrome.app"
    alias firefox="open -a /Applications/Firefox.app"
    alias ls="ls -FG"
    alias la="ls -aFG"
    alias ll="ls -FGl"
    alias pushDotfiles="open -a /Applications/Safari.app https://www.github.com/yuu-kobayashi/dotfiles/ && cd ${HOME}/dotfiles/ && git add . && git commit && git push origin master"
    ;;
  linux*)
    # linuxのときに読み込まれる設定
    ;;
esac

### Added by Zplugin's installer
source "${HOME}/.zplugin/bin/zplugin.zsh"
autoload -Uz _zplugin
(( ${+_comps} )) && _comps[zplugin]=_zplugin
### End of Zplugin's installer chunk

# plugins
zplugin load zsh-users/zsh-syntax-highlighting # コマンドが存在するかどうかで色付けしてくれる
