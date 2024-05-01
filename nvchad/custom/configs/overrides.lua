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
    "python",
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
    -- "deno",
    "prettier",

    -- c/cpp stuff
    "clangd",
    "clang-format",

    -- Bash
    "bash-language-server",
    "shellcheck",
    "shfmt",

    -- Vim script
    "vim-language-server",

    -- Go
    "goimports",
    "gopls",

    -- Rust
    -- Rust言語をインストールするときに一緒にインストールされるからそれを使う

    -- Python
    "python-lsp-server",
    "autopep8",

    -- Angular
    -- "typescript-language-server",
    "angular-language-server",
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
