" 文字コードの設定
" バッファ内での文字コードの指定
set encoding=utf-8
" fileencodingの設定を適用できないとき用の対策
set modifiable
" 書き込むときのデフォルトの文字コードの指定
set fileencoding=utf-8
" 読み込むときの文字コードの指定(左の方が優先度が高い)
set fileencodings=utf-8,cp932,sjis,euc-jp


" 改行コードの設定
" 左の方が優先度が高い
if has('win64')
    " Windowsのときは<CR+LF>を最優先する
    set fileformats=dos,unix,mac
else
    " それ以外のときは<CR>を最優先する
    set fileformats=unix,dos,mac
endif


" 行番号を表示する
set number


" シンタックスハイライトをオンにする
syntax enable


" カラースキームの設定
" ファイルが無かったらダウンロードする関数を定義
function! g:DownloadIfNotFileReadable(file_path, remote_url) abort
    if filereadable(a:file_path)
        " ファイルがローカルに存在していたら何もしない
        return
    endif
    " OS別のcurlコマンド
    let l:curl_command = has('win64') ? 'curl.exe' : 'curl'
    " ファイルをダウンロードする
    let l:message = system(l:curl_command . ' -Lo ' . a:file_path . ' --create-dirs ' . a:remote_url)
    if l:message !~# '.*% Total.*% Received.*% Xferd.*'
        " ダウンロードに失敗したらエラーメッセージを表示する
        echo 'error: ' . l:message
    endif
endfunction

" neovimから読み込まれていたときは処理しない
if !has('nvim')
    " gruvboxがなかったらダウンロードする
    call g:DownloadIfNotFileReadable(has('win64') ? $HOME . '\vimfiles\colors\gruvbox.vim' : $HOME . '/.vim/colors/gruvbox.vim', 'https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')

    " カラースキームをgruvboxに指定する
    colorscheme gruvbox
endif

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
" backspaceしたときに一度に消されるスペースの数らしい
set softtabstop=4

" 改行したときにインデントしてくれる
set autoindent

" インデントがスマートになる（らしい）
set smartindent


" 全角文字をちゃんと表示する
set ambiwidth=double


" 折り返したときの設定
" 折り返したときにインデントする
set breakindent
" 折り返したときの追加のインデントの深さを指定する
set breakindentopt=shift:0
" 単語単位で折り返す
set linebreak

set mouse=

set backspace=start


" 入力切替として設定しているキーショートカットに反応しないようにする
noremap <C-S-j> <Nop>
noremap! <C-S-j> <Nop>
tnoremap <C-S-j> <Nop>
noremap <C-S-:> <Nop>
noremap! <C-S-:> <Nop>
tnoremap <C-S-:> <Nop>
noremap <C-S-;> <Nop>
noremap! <C-S-;> <Nop>
tnoremap <C-S-;> <Nop>

" ノーマルモードで「:」と「;」を打ち間違えても「:」と入力されるようにする
nnoremap ; :


" 言語ごとの設定
" インデントの設定
filetype indent on
autocmd FileType java setlocal shiftwidth=2 softtabstop=2
autocmd FileType html setlocal shiftwidth=2 softtabstop=2
autocmd FileType css setlocal shiftwidth=2 softtabstop=2
autocmd FileType javascript setlocal shiftwidth=2 softtabstop=2
autocmd FileType javascriptreact setlocal shiftwidth=2 softtabstop=2
autocmd FileType typescript setlocal shiftwidth=2 softtabstop=2
autocmd FileType typescriptreact setlocal shiftwidth=2 softtabstop=2
autocmd FileType json setlocal shiftwidth=2 softtabstop=2

" 文字コードの設定
autocmd FileType dosbatch setlocal fileencoding=cp932


" ローカルのvimの設定を読み込む(set columnsの上書きとかをする)
" ファイルがあるときだけ読み込む
if filereadable($HOME . '/.vimrc_local')
    source ~/.vimrc_local
endif


" :Tc で今いるウィンドウにターミナルを開く
command Tc term ++curwin

" :Python でpythonを開く（pythonを閉じたらウィンドウも閉じる）
command Python term ++curwin ++close python


" gVimのフォントの設定
set guifont=Yomogi_Nerd_Font:h12


" スワップファイルを作らないようにする
set noswapfile
