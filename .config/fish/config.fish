# 文字コードをUTF-8にする
set -x LC_ALL ja_JP.UTF-8


# linuxbrewの設定
if [ -d /home/linuxbrew/.linuxbrew ]
    set -x HOMEBREW_PREFIX "/home/linuxbrew/.linuxbrew"
    set -x HOMEBREW_CELLAR "/home/linuxbrew/.linuxbrew/Cellar"
    set -x HOMEBREW_REPOSITORY "/home/linuxbrew/.linuxbrew/Homebrew"
    set -x HOMEBREW_SHELLENV_PREFIX "/home/linuxbrew/.linuxbrew"
    set -x PATH {$PATH} "/home/linuxbrew/.linuxbrew/bin" "/home/linuxbrew/.linuxbrew/sbin"
    set -x MANPATH {$MANPATH} "/home/linuxbrew/.linuxbrew/share/man"
    set -x INFOPATH {$INFOPATH} "/home/linuxbrew/.linuxbrew/share/info"
end


if ! [ -z {$SSH_CONNECTION} ]
    # SSH接続の場合
    if type tmux > /dev/null 2>&1
        if [ -z {$TMUX} ]
            # tmuxのセッション外だったら
            if tmux list-sessions > /dev/null 2>&1
                # すでにセッションが存在したらそれにアタッチする
                exec tmux -u attach
            else
                # セッションが存在しなかったら新しく作ってアタッチする
                exec tmux -u
            end
        end
    end
end


# Goの設定
set -x GOPATH {$HOME}/.go


# パスの設定
if ! [ -e {$HOME}/myCommands ]
    mkdir {$HOME}/myCommands
end
set -x PATH {$PATH} {$HOME}/myCommands

if [ -d {$HOME}/.sdkman/candidates/java/current/bin ]
    set -x PATH {$PATH} {$HOME}/.sdkman/candidates/java/current/bin
end

if [ -d {$HOME}/.sdkman/candidates/kotlin/current/bin ]
    set -x PATH {$PATH} {$HOME}/.sdkman/candidates/kotlin/current/bin
end

if [ -d {$HOME}/.local/bin ]
    set -x PATH {$PATH} {$HOME}/.local/bin
end

if [ -d {$HOME}/.cargo/bin ]
    set -x PATH {$PATH} {$HOME}/.cargo/bin
end

if [ -d {$HOME}/.nimble/bin ]
    set -x PATH {$PATH} {$HOME}/.nimble/bin
end

if [ -d {$HOME}/.go/bin ]
    set -x PATH {$PATH} {$GOPATH}/bin
end


# prompt_pwdでパスを省略しない
set -g fish_prompt_pwd_dir_length 0


# neovimに必要な設定
set -x XDG_CONFIG_HOME {$HOME}/.config
set -x XDG_CACHE_HOME {$HOME}/.cache


# デフォルトのエディタの設定
# neovim、vim、viの順番でインストールされているか確認して、されていたらそれに設定する
if type nvim > /dev/null 2>&1
    set -x EDITOR nvim
    set -x VISUAL nvim
else if type vim > /dev/null 2>&1
    set -x EDITOR vim
    set -x VISUAL vim
else if type vi > /dev/null 2>&1
    set -x EDITOR vi
    set -x VISUAL vi
else
    echo 'nvimもvimもviもありませんでした'
end


# HomebrewのAnalyticsを停止
set -x HOMEBREW_NO_ANALYTICS 1
eval (/opt/homebrew/bin/brew shellenv)


# エイリアスの設定
alias less "less -cmN"
alias del "delete"


# OS毎の設定
switch (uname) # なぜかlinterに引っかかる
    case Darwin # Macのとき
        alias safari "open -a /Applications/Safari.app"
        alias chrome "open -a /Applications/Google\ Chrome.app"
        alias firefox "open -a /Applications/Firefox.app"
        alias ls "ls -FG"
        alias la "ls -aFG"
        alias ll "ls -FGl"
    case Linux # Linuxのとき
        alias ls "ls -F --color=auto"
        alias la "ls -aF --color=auto"
        alias ll "ls -Fl --color=auto"
end

# Windows Subsystem for Linuxのとき
if uname -r | grep -i Microsoft > /dev/null 2>&1
    alias firefox "/mnt/c/Program\ Files/Mozilla\ Firefox/firefox.exe"
end
