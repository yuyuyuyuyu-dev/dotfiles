package dev.yuyuyuyuyu.claudeghadmissionhook

private val SHIPPED_COMMANDS =
    listOf(
        "add",
        "am",
        "annotate",
        "apply",
        "archive",
        "backfill",
        "bisect",
        "blame",
        "branch",
        "bugreport",
        "bundle",
        "cat-file",
        "check-attr",
        "check-ignore",
        "check-mailmap",
        "check-ref-format",
        "checkout",
        "checkout--worker",
        "checkout-index",
        "cherry",
        "cherry-pick",
        "clean",
        "clone",
        "column",
        "commit",
        "commit-graph",
        "commit-tree",
        "config",
        "count-objects",
        "credential",
        "credential-cache",
        "credential-cache--daemon",
        "credential-osxkeychain",
        "credential-store",
        "daemon",
        "describe",
        "diagnose",
        "diff",
        "diff-files",
        "diff-index",
        "diff-pairs",
        "diff-tree",
        "difftool",
        "difftool--helper",
        "fast-export",
        "fast-import",
        "fetch",
        "fetch-pack",
        "filter-branch",
        "fmt-merge-msg",
        "for-each-ref",
        "for-each-repo",
        "format-patch",
        "fsck",
        "fsck-objects",
        "fsmonitor--daemon",
        "gc",
        "get-tar-commit-id",
        "grep",
        "gui--askpass",
        "gui--askyesno",
        "hash-object",
        "help",
        "history",
        "hook",
        "http-backend",
        "http-fetch",
        "http-push",
        "imap-send",
        "index-pack",
        "init",
        "init-db",
        "interpret-trailers",
        "last-modified",
        "log",
        "ls-files",
        "ls-remote",
        "ls-tree",
        "mailinfo",
        "mailsplit",
        "maintenance",
        "merge",
        "merge-base",
        "merge-file",
        "merge-index",
        "merge-octopus",
        "merge-one-file",
        "merge-ours",
        "merge-recursive",
        "merge-recursive-ours",
        "merge-recursive-theirs",
        "merge-resolve",
        "merge-subtree",
        "merge-tree",
        "mergetool",
        "mktag",
        "mktree",
        "multi-pack-index",
        "mv",
        "name-rev",
        "notes",
        "p4",
        "pack-objects",
        "pack-redundant",
        "pack-refs",
        "patch-id",
        "pickaxe",
        "prune",
        "prune-packed",
        "pull",
        "push",
        "quiltimport",
        "range-diff",
        "read-tree",
        "rebase",
        "receive-pack",
        "reflog",
        "refs",
        "remote",
        "remote-ext",
        "remote-fd",
        "remote-ftp",
        "remote-ftps",
        "remote-http",
        "remote-https",
        "repack",
        "replace",
        "replay",
        "repo",
        "request-pull",
        "rerere",
        "reset",
        "restore",
        "rev-list",
        "rev-parse",
        "revert",
        "rm",
        "send-email",
        "send-pack",
        "sh-i18n--envsubst",
        "shell",
        "shortlog",
        "show",
        "show-branch",
        "show-index",
        "show-ref",
        "sparse-checkout",
        "stage",
        "stash",
        "status",
        "stripspace",
        "submodule",
        "submodule--helper",
        "subtree",
        "switch",
        "symbolic-ref",
        "tag",
        "unpack-file",
        "unpack-objects",
        "update-index",
        "update-ref",
        "update-server-info",
        "upload-archive",
        "upload-archive--writer",
        "upload-pack",
        "var",
        "verify-commit",
        "verify-pack",
        "verify-tag",
        "version",
        "web--browse",
        "whatchanged",
        "worktree",
        "write-tree",
    )

class Alias(
    val globals: List<String>,
    val name: String,
    val written: String,
    val rest: String,
)

class Unresolved(
    val reason: String,
    cause: Throwable,
) : Exception(reason, cause)

fun checkAlias(invocation: Invocation): Verdict? {
    val name = invocation.name
    val globals = invocation.globals
    val unreadable = globals.find { !literal(it) }
    return when {
        !couldBeAlias(name) -> null
        unreadable != null ->
            Verdict.Deny(
                "`git $name` is not a command git ships, so it may be an alias, and ${shown(unreadable)} comes ahead " +
                    "of it and out of a shell expansion, so the git configuration that would say so cannot be " +
                    "looked up. Write it out literally.",
            )
        else ->
            Verdict.PassIfNotAlias(
                Alias(globals.map { it.value }, name, written(globals), written(invocation.rest)),
            )
    }
}

private fun couldBeAlias(name: String) =
    name !in SHIPPED_COMMANDS && name.isNotEmpty() && !name.startsWith('-') && name.none { it.isUnicodeWhiteSpace() }

private fun written(tokens: List<Token>) = tokens.joinToString(" ") { it.raw }

fun resolve(
    alias: Alias,
    directory: String?,
): List<String> {
    val named = alias.name
    val keys = """^alias\.(${named.lowercase()}|$named\.command)$"""
    val found =
        try {
            settings(alias.globals, keys, directory)
        } catch (unanswered: Unanswered) {
            throw Unresolved(
                "`git $named` is not a command git ships, so it may be an alias, and whether it is one could not be " +
                    "looked up: `git config` ${unanswered.failure}.",
                unanswered,
            )
        }
    return found
        .split('\u0000')
        .filter { '\n' in it }
        .map { aliased(it.substringAfter('\n').trimWhiteSpace(), alias) }
}

private fun aliased(
    value: String,
    alias: Alias,
): String {
    val script = value.removePrefix("!")
    return if (value.startsWith('!')) "$script ${alias.rest}" else "git ${alias.written} $value ${alias.rest}"
}
