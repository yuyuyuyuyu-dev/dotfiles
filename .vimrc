" 文字コードの設定
" バッファ内での文字コードの指定
set encoding=utf-8
" 書き込むときのデフォルトの文字コードの指定
set fileencoding=utf-8
" 読み込むときの文字コードの指定(左の方が優先度が高い)
set fileencodings=utf-8,cp932,sjis,euc-jp


" 改行コードの設定
" 左の方が優先度が高い
if system("bash -c 'echo -n $(uname -r)'") =~ 'Microsoft$'
    " Windows(Sub system for Linux)のときは<CR+LF>を最優先する
    set fileformats=dos,unix,mac
else
    " それ以外のときは<CR>を最優先する
    set fileformats=unix,dos,mac
endif


" 行番号を表示する
set number


" シンタックスハイライトをオンにする
syntax enable 


" たぶんcolortarmをtrue colorの256色にする設定
" いらないっぽい？
" set t_Co=256


" カラースキームの設定
" neovimの設定ファイルが入ってるフォルダを取得する
let s:neovim_config_dir = empty($XDG_CONFIG_HOME) ? expand('~/.config/nvim') : $XDG_CONFIG_HOME . '/nvim'

" gruvboxがなかったら用意する
if !filereadable(expand('~/.vim/colors/gruvbox.vim'))
    " ~/.vimが無かったら作る
    if !isdirectory(expand('~/.vim'))
        call mkdir(expand('~/.vim'), 'p')
    endif

    " neovimの方にあったらリンクを貼る
    if filereadable(s:neovim_config_dir . '/colors/gruvbox.vim')
        " $XDG_CONFIG_HOMEが設定されているかによってリンク元を変える
        if empty($XDG_CONFIG_HOME)
            call system('ln -fns ~/.config/nvim/colors ~/.vim/')
        else
            call system('ln -fns $XDG_CONFIG_HOME/nvim/colors ~/.vim/')
        endif
    else
        " 無かったらダウンロードする
        " ダウンロード先のディレクトリが無かったら作る
        if !isdirectory(expand('~/.vim/colors'))
            call mkdir(expand('~/.vim/colors'))
        endif
        " gruvboxをダウンロードする
        call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o ~/.vim/colors/gruvbox.vim')
    endif
endif

" カラースキームの設定
colorscheme gruvbox

" ライトテーマを使う
set background=light


"インデントの設定
" インデントにスペースを使う
set expandtab

" Tab文字の長さの設定
set tabstop=4

" 一つのインデントのスペースの数
set shiftwidth=4

" よくわからないけどとりあえず書いておく。
set softtabstop=4  

" 改行したときにインデントしてくれる
set autoindent

" インデントがスマートになる（らしい）
set smartindent


" undo履歴を保持し続ける
if has('persistent_undo')
    set undodir=~/.vimundo
    set undofile
endif


" 全角文字をちゃんと表示する
set ambiwidth=double
