" 文字コードにutf-8を指定する
set encoding=utf-8
set fileencoding=utf-8
" 読み込み時の文字コードを指定する
" 左の方が優先度が高い
set fileencodings=utf-8,cp932,sjis,euc-jp

" 改行コードの設定
" 左の方が優先度が高い
if system("bash -c 'echo -n $(uname)'") ==# 'Windows'
  set fileformats=dos,unix,mac
else
  set fileformats=unix,dos,mac
endif

" 行番号を表示する
set number

" シンタックスハイライトをオンにする
syntax enable 

" たぶんcolortarmをtrue colorの256色にする設定
set t_Co=256

" カラースキーマにgruvboxを設定
" なかったらダウンロード
" そのうち書く
colorscheme gruvbox
" ライトテーマを使う
set background=light

"インデントの設定
set expandtab      "インデントをtab文字で表現する
set tabstop=4      "tab1つの長さをスペース4個分にする
set shiftwidth=2   "自動で挿入されるtabの長さをスペース2個分にする
set softtabstop=2  "よくわからないけどとりあえず書いておく。
set autoindent     "オートインデント
set smartindent    "いい感じにインデントしてくれる

" set completeopt-=preview


" undo履歴を保持し続ける
if has('persistent_undo')
  set undodir=~/.vimundo
  set undofile
endif


" 全角文字をちゃんと表示する
set ambiwidth=double


" test
" if isdirectory('%:h/.vimundo')
"   echo 'true'
" else
"   echo 'false'
" endif
