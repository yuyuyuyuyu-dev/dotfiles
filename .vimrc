" 文字コードにutf-8を指定する
set encoding=utf-8

" 行番号を表示する
set number

" シンタックスハイライトをオンにする
syntax enable 

" たぶんcolortarmをtrue colorの256色にする設定
set t_Co=256

" カラースキーマにgruvboxを設定
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

set completeopt-=preview


" undo履歴を保持し続ける
if has('persistent_undo')
  set undodir=~/.vimundo
  set undofile
endif


" 全角文字をちゃんと表示する
set ambiwidth=double


" シェルをfishに設定する
if system('echo -n $SHELL') =~ "fish$"
  set shell=call system('echo -n (which fish)')
else
  set shell=call system('echo -n $(which fish)')
endif


" test
" if isdirectory('%:h/.vimundo')
"   echo 'true'
" else
"   echo 'false'
" endif
