# claude-gh-admission-hook

A hook that restricts how Claude Code reaches GitHub.

## Why

Claude Code now starts in auto mode by default, and I no longer expect to prevent every unintended change to my local files. So I decided to accept that risk locally, and to defend the remote side properly instead.

Online storage is not reachable from my command line, so it was never a problem for me. The only problem left was GitHub access through the `gh` command. That is why I wrote this hook.

## What

This is a hook you register in `~/.claude/settings.json`. It keeps GitHub access through the `gh` command read-only, and asks you before the only two writes it allows: creating a pull request and editing one. Pushing to a topic branch is not restricted, because a pull request needs one.

Creating a tag is denied as well, because a tag can start a release workflow without me. (Stopping only the push turned out to be hard, so I stopped the creation instead.)

It also denies writes sent straight to the GitHub API with `curl` or `wget`, including the ones that carry a token taken from `gh`.

## How

```bash
cargo install --git https://github.com/yuyuyuyuyu-dev/dotfiles --locked claude-gh-admission-hook
```

After installing it, register it in `~/.claude/settings.json` as a `PreToolUse` hook. Asking Claude Code to do that is probably the easiest way, since it appears to carry documentation on the current structure of the file.
