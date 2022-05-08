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
" gruvboxがなかったらダウンロードする
" neovimから読み込まれていたときはダウンロードしない
if !has('nvim')
    let s:gruvbox_path = has('win64') ? $HOME . '\vimfiles\colors\gruvbox.vim' : $HOME . '/.vim/colors/gruvbox.vim'
    if !filereadable(s:gruvbox_path)
        let s:curl_command = has('win64') ? 'curl.exe -fLo ' : 'curl -o '
        call system(s:curl_command . s:gruvbox_path . ' --create-dirs https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')
    endif
endif
" カラースキームをgruvboxに指定する
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


" 行末にセミコロンを挿入する
function! InsertEndSemicolon() abort
    " カーソルの現在位置を取得する
    let s:currentPosition = getpos('.')

    " 行末にセミコロンがなかったら挿入する
    if !(getline('.') =~ ';$')
        execute ':normal A;'
    endif

    " カーソル位置を戻す
    call setpos('.', s:currentPosition)
endfunction

" ノーマルモードでセミコロンを押すとInsertEndSemicolon()が呼ばれるようにする
nnoremap <silent> ; :call InsertEndSemicolon()<CR>
" インサートモードでセミコロン２度押しでその場にセミコロンを入力できるようにする
inoremap <silent> ;; ;
" インサートモードでセミコロン押してからエスケープキーを押すと行末にセミコロンを挿入してからノーマルモードに戻る
inoremap <silent> ;<ESC> <ESC>:call InsertEndSemicolon()<CR>


" 言語ごとのインデントの設定
filetype indent on
autocmd FileType java setlocal shiftwidth=2 softtabstop=2


" ローカルのvimの設定を読み込む(set columnsの上書きとかをする)
" ファイルがあるときだけ読み込む
if filereadable($HOME . '/.vimrc_local')
    source ~/.vimrc_local
endif
