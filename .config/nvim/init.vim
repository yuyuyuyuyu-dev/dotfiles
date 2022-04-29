" XDG Base Directory Specification
let s:config_home = empty($XDG_CONFIG_HOME) ? has('win64') ? $LOCALAPPDATA : expand('~/.config') : $XDG_CONFIG_HOME

let s:share_nvim_dir = has('win64') ? $LOCALAPPDATA . '/nvim-data' : $HOME . '/.local/share/nvim'


" シェルを指定する
let s:bash_path = system("bash -c 'echo -n $(which bash)'")
let s:fish_path = system("bash -c 'echo -n $(which fish)'")
" bash
if s:bash_path ==# "/usr/bin/bash"
    set shell=/usr/bin/bash
endif
" fish
" Linux
if s:fish_path ==# "/usr/bin/fish"
    set shell=/usr/bin/fish
endif
" Mac
if s:fish_path ==# "/usr/local/bin/fish"
    set shell=/usr/local/bin/fish
endif
" Linuxbrew
if s:fish_path ==# "/home/linuxbrew/.linuxbrew/bin/fish"
    set shell=/home/linuxbrew/.linuxbrew/bin/fish
endif
" Windows
if has('win64')
    set shell=C:\WINDOWS\system32\cmd.exe
endif


" vim-plugがインストールされていなかったらインストールする
if !filereadable(s:share_nvim_dir . '/site/autoload/plug.vim')
    if has('win64')
        " Windowsの場合
        call system('curl.exe -fLo %LOCALAPPDATA%\nvim-data\site\autoload\plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')
        source ~/AppData/Local/nvim/plugin_settings/vim-plug_setting.vim
    else
        " Windows以外の場合
        call system('curl -fLo ~/.local/share/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')
        source ~/.config/nvim/plugin_settings/vim-plug_setting.vim
    endif
    " ついでにプラグインもインストールする
    PlugInstall
endif

" カラースキームの"gruvbox"が無かったら用意する
if !filereadable(s:config_home . '/nvim/colors/gruvbox.vim')
    " ~/.vim/colorsの中にあったらリンクを貼る
    if filereadable($HOME . '/.vim/colors/gruvbox.vim')
        " call system(has('win64') ? '')
    else
    endif
endif

if has('win64')
    if !filereadable(expand('~/AppData/Local/nvim/colors/gruvbox.vim'))
        call system('curl.exe -fLo %LOCALAPPDATA%\nvim\colors\gruvbox.vim --create-dirs https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')
    endif
else
    if !filereadable(s:config_home . '/nvim/colors/gruvbox.vim')
        " ~/.vim/colorsの中にあったらリンクを貼る
        if filereadable(expand('~/.vim/colors/gruvbox.vim'))
            " $XDG_CONFIG_HOMEが設定されているかによってリンク先を変える
            if empty($XDG_CONFIG_HOME)
                call system('ln -fns ~/.vim/colors ~/.config/nvim/')
            else
                call system('ln -fns ~/.vim/colors $XDG_CONFIG_HOME/nvim/')
            endif
        else
            " ~/.vim/colorsの中にも無かったらダウンロードする
            " ダウンロード先のフォルダが無かったら作る
            if !isdirectory(s:config_home . '/nvim/colors')
                call mkdir(s:config_home . '/nvim/colors', 'p')
            endif

            " $XDG_CONFIG_HOMEが設定されているかによってダウンロード先を変える
            if empty($XDG_CONFIG_HOME)
                call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o ~/.config/nvim/colors/gruvbox.vim')
            else
                call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o $XDG_CONFIG_HOME/nvim/colors/gruvbox.vim')
            endif
        endif
    endif
endif


" vimの設定を読み込む
if has('win64')
    source ~\_vimrc
else
    source ~/.vimrc
endif
