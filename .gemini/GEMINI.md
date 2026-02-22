# GEMINI.md

## 基本ルール

これらのルールを基本とします
プロジェクト固有のルールでこれらと競合するものがあった場合、プロジェクト固有のルールを優先してください

- 回答は日本語で行うこと
- mainブランチでのコミットは禁止とする
  - 規約に反したコードが紛れ込んでしまうのを極力防ぐため
  - 作業ブランチでのコミットは許可する
    - プルリクエストによってコードレビューを責任者に強制できるため
- ソースコード内にコメントを書くことは禁止とする
  - 常に説明的な変数名や関数名を心がけ、Self-documenting codeを徹底すること
    - もしひとまとまりの処理の内容を説明するためにコメントを書きたくなった場合、プライベート関数を作りその関数名で説明することを推奨する
- ビルドは原則として行わないこと
  - 並行でビルドがいくつも走ってPCが重くなってしまうのを防ぐため
- 作業完了時にはフォーマッターを実行しコード整形した上で、リンターを実行して文法ミスがないか確認すること
- インターフェースおよびテストコードの作成、編集、削除は禁止とする
  - それらはコードの品質保証をする上で最も大切なものであり、品質に対して責任を負う者が管理するべきであるため
- 正常系以外は積極的にクラッシュさせる設計を心がけること
  - 実際にクラッシュさせることで、ハンドリングするべきエラーかの判断をしやすくするため

## コミット規約

Angularのコミットメッセージガイドラインに従ってください

[angular/contributing-docs/commit-message-guidelines.md](https://github.com/angular/angular/blob/main/contributing-docs/commit-message-guidelines.md) から引用

> # Commit Message Format
> 
> We have very precise rules over how our Git commit messages must be formatted.
> This format leads to **easier to read commit history** and makes it analyzable for changelog generation.
> 
> Each commit message consists of a **header**, a **body**, and a **footer**.
> 
> ```
> <header>
> <BLANK LINE>
> <body>
> <BLANK LINE>
> <footer>
> ```
> 
> The `header` is mandatory and must conform to the [Commit Message Header](#commit-header) format.
> 
> The `body` is mandatory for all commits except for those of type "docs".
> When the body is present it must be at least 20 characters long and must conform to the [Commit Message Body](#commit-body) format.
> 
> The `footer` is optional. The [Commit Message Footer](#commit-footer) format describes what the footer is used for and the structure it must have.
> 
> ## <a name="commit-header"></a>Commit Message Header
> 
> ```
> <type>(<scope>): <short summary>
>   │       │             │
>   │       │             └─⫸ Summary in present tense. Not capitalized. No period at the end.
>   │       │
>   │       └─⫸ Commit Scope: animations|bazel|benchpress|common|compiler|compiler-cli|core|
>   │                          elements|forms|http|language-service|localize|platform-browser|
>   │                          platform-browser-dynamic|platform-server|router|service-worker|
>   │                          upgrade|zone.js|packaging|changelog|docs-infra|migrations|
>   │                          devtools
>   │
>   └─⫸ Commit Type: build|ci|docs|feat|fix|perf|refactor|test
> ```
> 
> The `<type>` and `<summary>` fields are mandatory, the `(<scope>)` field is optional.
> 
> ### Type
> 
> Must be one of the following:
> 
> | Type         | Description                                                                                         |
> | ------------ | --------------------------------------------------------------------------------------------------- |
> | **build**    | Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm) |
> | **ci**       | Changes to our CI configuration files and scripts (examples: Github Actions, SauceLabs)             |
> | **docs**     | Documentation only changes                                                                          |
> | **feat**     | A new feature                                                                                       |
> | **fix**      | A bug fix                                                                                           |
> | **perf**     | A code change that improves performance                                                             |
> | **refactor** | A code change that neither fixes a bug nor adds a feature                                           |
> | **test**     | Adding missing tests or correcting existing tests                                                   |
> 
> ### <a name="scope"></a> Scope
> 
> The scope should be the name of the npm package affected (as perceived by the person reading the changelog generated from commit messages).
> 
> The following is the list of supported scopes:
> 
> - `animations`
> - `benchpress`
> - `common`
> - `compiler`
> - `compiler-cli`
> - `core`
> - `dev-infra`
> - `devtools`
> - `docs-infra`
> - `elements`
> - `forms`
> - `http`
> - `language-service`
> - `language-server`
> - `localize`
> - `migrations`
> - `platform-browser`
> - `platform-browser-dynamic`
> - `platform-server`
> - `router`
> - `service-worker`
> - `upgrade`
> - `vscode-extension`
> - `zone.js`
> 
> There are currently a few exceptions to the "use package name" rule:
> 
> - `dev-infra`: used for dev-infra related changes within the directories /scripts and /tools
> 
> - `docs-infra`: used for infrastructure changes to the docs-app (angular.dev) within the `/adev` directory, such as application code, tooling, or configuration. **For modifications to documentation content (e.g., editing a `.md` file), use `docs:` without a scope instead.**
> 
> - `migrations`: used for changes to the `ng update` migrations.
> 
> - `devtools`: used for changes in the [browser extension](../devtools/README.md).
> 
> - none/empty string: useful for `test` and `refactor` changes that are done across all packages (e.g. `test: add missing unit tests`) and for docs changes that are not related to a specific package (e.g. `docs: fix typo in tutorial`).
> 
> ### Summary
> 
> Use the summary field to provide a succinct description of the change:
> 
> - use the imperative, present tense: "change" not "changed" nor "changes"
> - don't capitalize the first letter
> - no dot (.) at the end
> 
> ## <a name="commit-body"></a>Commit Message Body
> 
> Just as in the summary, use the imperative, present tense: "fix" not "fixed" nor "fixes".
> 
> Explain the motivation for the change in the commit message body. This commit message should explain _why_ you are making the change.
> You can include a comparison of the previous behavior with the new behavior in order to illustrate the impact of the change.
> 
> ## <a name="commit-footer"></a>Commit Message Footer
> 
> The footer can contain information about breaking changes and deprecations and is also the place to reference GitHub issues and other PRs that this commit closes or is related to.
> For example:
> 
> ```
> BREAKING CHANGE: <breaking change summary>
> <BLANK LINE>
> <breaking change description + migration instructions>
> <BLANK LINE>
> <BLANK LINE>
> Fixes #<issue number>
> ```
> 
> or
> 
> ```
> DEPRECATED: <what is deprecated>
> <BLANK LINE>
> <deprecation description + recommended update path>
> <BLANK LINE>
> <BLANK LINE>
> Closes #<pr number>
> ```
> 
> Breaking Change section should start with the phrase `BREAKING CHANGE: ` followed by a _brief_ summary of the breaking change, a blank line, and a detailed description of the breaking change that also includes migration instructions.
> 
> Similarly, a Deprecation section should start with `DEPRECATED: ` followed by a short description of what is deprecated, a blank line, and a detailed description of the deprecation that also mentions the recommended update path.
> 
> ## Revert commits
> 
> If the commit reverts a previous commit, it should begin with `revert: `, followed by the header of the reverted commit.
> 
> The content of the commit message body should contain:
> 
> - information about the SHA of the commit being reverted in the following format: `This reverts commit <SHA>`,
> - a clear description of the reason for reverting the commit message.
> 
> [angularjs-commit-message-format]: https://docs.google.com/document/d/1QrDFcIiPjSLDn3EL15IJygNPiHORgU1_OOAqWjiDU5Y/edit#

---

The MIT License

Copyright (c) 2010-2026 Google LLC. https://angular.dev/license

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---