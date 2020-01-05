# 文字コードをUTF-8にする
set -x LC_ALL ja_JP.UTF-8

# パスの設定
set -x PATH {$PATH} {$HOME}/myCommands

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

# エイリアスの設定
alias less "less -N"
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
    alias pushDotfiles "open -a /Applications/Safari.app https://www.github.com/yuu-kobayashi/dotfiles/ && cd {$HOME}/dotfiles/ && git add . && git commit && git push origin master"
  case Linux # Linuxのとき
    alias ls "ls -F --color=auto"
    alias la "ls -aF --color=auto"
    alias ll "ls -Fl --color=auto"
end

# Windows Subsystem for Linuxのとき
if uname -r | grep -i Microsoft > /dev/null 2>&1
  alias firefox "/mnt/c/Program\ Files/Mozilla\ Firefox/firefox.exe"
end
