require "nvchad.options"

vim.o.fileencodings = "utf-8,sjis,cp932,ec-jp"

vim.o.fileformats = vim.fn.has "win64" ~= 0 and "dos,unix,mac" or "unix,dos,mac"

vim.o.mouse = ""

vim.o.whichwrap = ""

vim.o.backspace = "start"

vim.api.nvim_create_augroup("hardtab_indent", { clear = true })
vim.api.nvim_create_autocmd({ "FileType" }, {
  pattern = "sh,gitconfig",
  group = "hardtab_indent",
  command = "setlocal noexpandtab",
})

vim.api.nvim_create_augroup("dosbatch fileencoding", { clear = true })
vim.api.nvim_create_autocmd({ "FileType" }, {
  pattern = "dosbatch",
  group = "dosbatch fileencoding",
  command = "setlocal fileencoding=sjis",
})
