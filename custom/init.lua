-- local autocmd = vim.api.nvim_create_autocmd

-- Auto resize panes when resizing nvim window
-- autocmd("VimResized", {
--   pattern = "*",
--   command = "tabdo wincmd =",
-- })

-- 参考資料
-- https://neovim.io/doc/user/lua-guide.html#lua-guide

-- set fileencodings=
vim.o.fileencodings = "utf-8,cp932,sjis,ec-jp"

-- set fileformats=
vim.o.fileformats = "dos,unix,mac"

vim.o.shell = "C:\\\\Users\\\\yu_kobayashi\\\\scoop\\\\shims\\\\nu.exe"
vim.o.shellcmdflag = "-c nu --login"

-- 文字幅の問題はターミナルエミュレータのHyperに丸投げする
-- vim.o.ambiwidth = "double"
-- vim.fn.setcellwidths {
--   {0x100, 0xFFFFFF, 2,},
-- }
