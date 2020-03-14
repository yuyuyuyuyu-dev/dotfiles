" vim-plugの設定
call plug#begin(stdpath('data') . '/plugged')
    " <C--><C-->でコメントインアウトを切替できるようにするプラグイン
    Plug 'tomtom/tcomment_vim'

    " ノーマルモードで cs"' ってやったら"を'に置換してくれる
    Plug 'tpope/vim-surround'

    " インデントを可視化してくれるやつ
    Plug 'nathanaelkane/vim-indent-guides'

    " 自動で閉じ括弧とか入力してくれるやつ
    Plug 'cohama/lexima.vim'

    " lintしてくれるやつ
    Plug 'w0rp/ale'

    " 補完に使うプラグインたち
    Plug 'prabirshrestha/async.vim'
    Plug 'prabirshrestha/vim-lsp'
    Plug 'mattn/vim-lsp-settings'
    Plug 'prabirshrestha/asyncomplete.vim'
    Plug 'prabirshrestha/asyncomplete-lsp.vim'
    Plug 'Shougo/neosnippet.vim'
    Plug 'Shougo/neosnippet-snippets'
    Plug 'prabirshrestha/asyncomplete-neosnippet.vim'
call plug#end()
