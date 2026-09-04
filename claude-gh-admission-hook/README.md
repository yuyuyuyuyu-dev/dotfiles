# claude-gh-admission-hook

Claude CodeによるGitHubへのアクセスを制限するフック。

## Why

Claude Codeのデフォルトのモードがauto modeになり、ローカルでの意図しない変更を完全に防ぐことはこの先難しいと感じました。
そこで、ローカルはAIに明け渡す代わりにリモートのファイルはしっかりと守る、という方針を取ることにしました。
オンラインストレージについてはコマンドラインからアクセスできるようにはしていないので私にとってそこは問題なくて、唯一の問題は `gh` コマンドによるGitHubへのアクセスでした。
なので私はこのフックを開発しました。

## What

`~/.claude/settings.json` に登録するフックとして、 `gh` コマンドによるGitHubへのアクセスを原則リードオンリー、プルリクエストの作成と編集だけaskに制限します。（プルリクエストを作るための前提となる、トピックブランチへのpushは制限しません。）
また、 `gh` コマンドからアクセストークンを取得して `curl` や `wget` でAPIを直接叩くことも禁止しています。

## How

```bash
cargo install --git https://github.com/yuyuyuyuyu-dev/dotfiles/tree/main/claude-gh-admission-hook
```

インストールした後に `~/.claude/settings.json` で `PreToolUse` フックとして登録してください。
登録はClaude Codeに依頼するのがいいでしょう。
内部的に `~/.claude/settings.json` の最新のデータ構造に関するドキュメントを持っているようなので。
