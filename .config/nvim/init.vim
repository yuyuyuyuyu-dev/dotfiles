" vim-plugがインストールされていなかったらインストールする
if !filereadable(expand('~/.local/share/nvim/site/autoload/plug.vim'))
  call system('curl -fLo ~/.local/share/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')
endif


" vimの設定を読み込む
source ~/.vimrc


" ここからneovim向けの設定
" デバッグ用
set cmdheight=2

" 開いたファイルがあるディレクトリに自動でcdする
set autochdir

" python3のパスを指定
if system('echo -n $SHELL') =~ "fish$"
  let g:python3_host_prog = system('echo -n (which python3)')
else
  let g:python3_host_prog = system('echo -n $(which python3)')
endif


" filetypeの追加
autocmd BufNewFile,BufRead *.fish setfiletype fish
autocmd BufNewFile,BufRead *.vim setfiletype vim
autocmd BufNewFile,BufRead *.swift setfiletype swift
autocmd BufNewFile,BufRead *.kt setfiletype kotlin


" vim-plugの設定
call plug#begin(stdpath('data') . '/plugged')
Plug 'tomtom/tcomment_vim'
Plug 'tpope/vim-surround'
Plug 'nathanaelkane/vim-indent-guides'
Plug 'cohama/lexima.vim'
Plug 'w0rp/ale'

Plug 'prabirshrestha/async.vim'
Plug 'prabirshrestha/vim-lsp'
Plug 'mattn/vim-lsp-settings'
Plug 'Shougo/deoplete.nvim', { 'do': ':UpdateRemotePlugins' }
Plug 'lighttiger2505/deoplete-vim-lsp'
Plug 'deoplete-plugins/deoplete-jedi'
Plug 'Shougo/neosnippet.vim'
Plug 'Shougo/neosnippet-snippets'
call plug#end()


" vim-indent-guidesの設定
let g:indent_guides_enable_on_vim_startup = 1
let g:indent_guides_start_level= 1
let g:indent_guides_guide_size = 0
autocmd VimEnter,Colorscheme * :hi IndentGuidesOdd	ctermbg=248
autocmd VimEnter,Colorscheme * :hi IndentGuidesEven	ctermbg=246


" aleの設定
let g:ale_sign_column_always = 1
let g:ale_sign_error = '☠︎'
let g:ale_sign_warning = '⚠︎'


"deopleteとneosnippetの競合を回避するための設定
imap <C-k> <Plug>(neosnippet_expand_or_jump)
smap <C-k> <Plug>(neosnippet_expand_or_jump)
xmap <C-k> <Plug>(neosnippet_expand_target)
if has('conceal')
	set conceallevel=2 concealcursor=niv
  endif


" deoplete.nvimの設定
let g:deoplete#enable_at_startup = 1


" pythonのneovimが見つからなくてdeopleteとかがエラーになっていたときは、pipでインストールした
" neovim, greenlet, pynvim, mspackgesをインストールし直したら治った
