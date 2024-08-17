require "nvchad.options"

-- add yours here!

-- local o = vim.o
-- o.cursorlineopt ='both' -- to enable cursorline!

-- Luaで設定を書く場合のNeovim公式ドキュメント
-- https://neovim.io/doc/user/lua-guide.html#lua-guide

-- 文字コードの設定
-- Vim scriptだと
-- set fileencodings=
vim.o.fileencodings = "utf-8,sjis,cp932,ec-jp"

-- 改行コードの設定
-- Vim scriptだと
-- set fileformats=
vim.o.fileformats = vim.fn.has "win64" ~= 0 and "dos,unix,mac" or "unix,dos,mac"

-- マウスに反応しないようにする
-- Vim scriptだと
-- set mouse=
vim.o.mouse = ""

-- 左右のカーソル移動で行をまたがないようにする
-- Vim scriptだと
-- set whichwrap=
vim.o.whichwrap = ""

-- インサートモードでのbackspaceで改行とautoindentを削除できないようにする
-- set backspace=
vim.o.backspace = "start"

-- filetypeがsh、gitconfigのときはインデントをハードタブにする
vim.api.nvim_create_augroup("hardtab_indent", { clear = true })
vim.api.nvim_create_autocmd({ "FileType" }, {
  pattern = "sh,gitconfig",
  group = "hardtab_indent",
  command = "setlocal noexpandtab",
})

-- filetypeがdosbatchのときは文字コードをShift_JISにする
vim.api.nvim_create_augroup("dosbatch fileencoding", { clear = true })
vim.api.nvim_create_autocmd({ "FileType" }, {
  pattern = "dosbatch",
  group = "dosbatch fileencoding",
  command = "setlocal fileencoding=sjis",
})
