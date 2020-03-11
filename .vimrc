" 文字コードの設定
" バッファ内での文字コードの指定
set encoding=utf-8
" 書き込むときのデフォルトの文字コードの指定
set fileencoding=utf-8
" 読み込むときの文字コードの指定(左の方が優先度が高い)
set fileencodings=utf-8,cp932,sjis,euc-jp


" 改行コードの設定
" 左の方が優先度が高い
if system("bash -c 'echo -n $(uname)'") ==# 'Windows'
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
set t_Co=256


" カラースキームの設定
" gruvboxがなかったらダウンロードする
if !filereadable(expand('~/.vim/colors/gruvbox.vim'))
  if !isdirectory(expand('~/.vim/colors'))
    " ダウンロード先のディレクトリが無かったら作る
    call system('mkdir -p ~/.vim/colors')
  endif
  " gruvboxをダウンロードする
  call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o ~/.vim/colors/gruvbox.vim')
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
set shiftwidth=2

" よくわからないけどとりあえず書いておく。
set softtabstop=2  

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
