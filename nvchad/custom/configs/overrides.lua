local M = {}

M.treesitter = {
  ensure_installed = {
    "vim",
    "lua",
    "html",
    "css",
    "javascript",
    "typescript",
    "tsx",
    "c",
    "markdown",
    "markdown_inline",
  },
  indent = {
    enable = true,
    -- disable = {
    --   "python"
    -- },
  },
}

M.mason = {
  ensure_installed = {
    -- lua stuff
    "lua-language-server",
    "stylua",

    -- web dev stuff
    "css-lsp",
    "html-lsp",
    "typescript-language-server",
    "deno",
    "prettier",

    -- Bash
    "bash-language-server",
    "shellcheck",
    "shfmt",

    -- Vim script
    "vim-language-server",
    -- Pythonの仮想環境が作れないとかでインストールに失敗する
    -- "vint",

    -- Go
    "goimports",
    "gopls",

    -- Rust
    "rust-analyzer",
    "rustfmt",

    -- C
    "clangd",
    -- Pythonの仮想環境が作れないとかでインストールに失敗する
    -- "clang-format",
  },
}

-- git support in nvimtree
M.nvimtree = {
  git = {
    enable = true,
  },

  renderer = {
    highlight_git = true,
    icons = {
      show = {
        git = true,
      },
    },
  },
}

return M
