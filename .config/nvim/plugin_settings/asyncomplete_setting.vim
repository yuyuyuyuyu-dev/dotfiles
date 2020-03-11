set completeopt+=preview

autocmd! CompleteDone * if pumvisible() == 0 | pclose | endif
