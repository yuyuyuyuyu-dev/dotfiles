require "nvchad.mappings"

-- add yours here

local map = vim.keymap.set

map("n", ";", ":", { desc = "CMD enter command mode" })
-- map("i", "jk", "<ESC>")

-- map({ "n", "i", "v" }, "<C-s>", "<cmd> w <cr>")

for _, key in ipairs { "<C-S-j>", "<C-S-;>" } do
  map({ "i", "n" }, key, "<Nop>", {
    desc = "入力切り替えのショートカットに反応しないようにする",
    nowait = true,
  })
end
