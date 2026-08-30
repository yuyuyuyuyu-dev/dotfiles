set encoding=utf-8
set modifiable
set fileencoding=utf-8
set fileencodings=utf-8,sjis,cp932,utf-16le,euc-jp

if has('win64')
    set fileformats=dos,unix,mac
else
    set fileformats=unix,dos,mac
endif

set number

syntax enable

function! g:DownloadIfNotFileReadable(file_path, remote_url) abort
    if filereadable(a:file_path)
        return
    endif
    let l:curl_command = has('win64') ? 'curl.exe' : 'curl'
    let l:message = system(l:curl_command . ' -Lo ' . a:file_path . ' --create-dirs ' . a:remote_url)
    if l:message !~# '.*% Total.*% Received.*% Xferd.*'
        echo 'error: ' . l:message
    endif
endfunction

if !has('nvim')
    call g:DownloadIfNotFileReadable(has('win64') ? $HOME . '\vimfiles\colors\gruvbox.vim' : $HOME . '/.vim/colors/gruvbox.vim', 'https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')

    colorscheme gruvbox
endif

set background=light

set expandtab

set tabstop=4

set shiftwidth=4

set softtabstop=4

set autoindent

set smartindent

set ambiwidth=double

set breakindent
set breakindentopt=shift:0
set linebreak

set mouse=

set backspace=start

noremap <C-S-j> <Nop>
noremap! <C-S-j> <Nop>
tnoremap <C-S-j> <Nop>
noremap <C-S-:> <Nop>
noremap! <C-S-:> <Nop>
tnoremap <C-S-:> <Nop>
noremap <C-S-;> <Nop>
noremap! <C-S-;> <Nop>
tnoremap <C-S-;> <Nop>

nnoremap ; :

filetype indent on
autocmd FileType java setlocal shiftwidth=2 softtabstop=2
autocmd FileType html setlocal shiftwidth=2 softtabstop=2
autocmd FileType css setlocal shiftwidth=2 softtabstop=2
autocmd FileType javascript setlocal shiftwidth=2 softtabstop=2
autocmd FileType javascriptreact setlocal shiftwidth=2 softtabstop=2
autocmd FileType typescript setlocal shiftwidth=2 softtabstop=2
autocmd FileType typescriptreact setlocal shiftwidth=2 softtabstop=2
autocmd FileType json setlocal shiftwidth=2 softtabstop=2

autocmd FileType dosbatch setlocal fileencoding=sjis

if filereadable($HOME . '/.vimrc_local')
    source ~/.vimrc_local
endif

command Tc term ++curwin

command Python term ++curwin ++close python

set guifont=Yomogi_Nerd_Font:h12

set noswapfile
