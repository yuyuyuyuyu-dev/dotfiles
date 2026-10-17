package dev.yuyuyuyuyu.claudeghadmissionhook

private val GH_FLAGS_WITHOUT_VALUE = listOf("--help", "--version")
private val AUTH_STATUS = listOf("auth", "status")
private const val SUBCOMMAND_DEPTH = 3

private val READ_GH_COMMANDS =
    listOf(
        listOf("browse"),
        listOf("status"),
        listOf("licenses"),
        listOf("completion"),
        listOf("help"),
        listOf("version"),
        listOf("auth", "status"),
        listOf("pr", "list"),
        listOf("pr", "status"),
        listOf("pr", "checks"),
        listOf("pr", "diff"),
        listOf("pr", "view"),
        listOf("pr", "checkout"),
        listOf("issue", "list"),
        listOf("issue", "status"),
        listOf("issue", "view"),
        listOf("repo", "list"),
        listOf("repo", "view"),
        listOf("repo", "clone"),
        listOf("repo", "gitignore"),
        listOf("repo", "license"),
        listOf("repo", "read-dir"),
        listOf("repo", "read-file"),
        listOf("repo", "deploy-key", "list"),
        listOf("repo", "autolink", "list"),
        listOf("repo", "autolink", "view"),
        listOf("release", "list"),
        listOf("release", "view"),
        listOf("release", "download"),
        listOf("release", "verify"),
        listOf("release", "verify-asset"),
        listOf("run", "list"),
        listOf("run", "view"),
        listOf("run", "watch"),
        listOf("run", "download"),
        listOf("workflow", "list"),
        listOf("workflow", "view"),
        listOf("cache", "list"),
        listOf("label", "list"),
        listOf("ruleset", "check"),
        listOf("ruleset", "list"),
        listOf("ruleset", "view"),
        listOf("secret", "list"),
        listOf("variable", "list"),
        listOf("variable", "get"),
        listOf("org", "list"),
        listOf("project", "list"),
        listOf("project", "view"),
        listOf("project", "field-list"),
        listOf("project", "item-list"),
        listOf("gist", "list"),
        listOf("gist", "view"),
        listOf("gist", "clone"),
        listOf("search", "code"),
        listOf("search", "commits"),
        listOf("search", "issues"),
        listOf("search", "prs"),
        listOf("search", "repos"),
        listOf("attestation", "download"),
        listOf("attestation", "trusted-root"),
        listOf("attestation", "verify"),
        listOf("gpg-key", "list"),
        listOf("ssh-key", "list"),
        listOf("config", "get"),
        listOf("config", "list"),
        listOf("alias", "list"),
        listOf("extension", "list"),
        listOf("extension", "search"),
        listOf("extension", "browse"),
        listOf("codespace", "list"),
        listOf("codespace", "view"),
        listOf("codespace", "logs"),
        listOf("discussion", "list"),
        listOf("discussion", "view"),
        listOf("skill", "list"),
        listOf("skill", "search"),
        listOf("skill", "preview"),
        listOf("agent-task", "list"),
        listOf("agent-task", "view"),
    )

private val BARE_READ_GH_COMMANDS =
    listOf(
        listOf("auth"),
        listOf("pr"),
        listOf("issue"),
        listOf("repo"),
        listOf("release"),
        listOf("run"),
        listOf("workflow"),
        listOf("cache"),
        listOf("label"),
        listOf("ruleset"),
        listOf("secret"),
        listOf("variable"),
        listOf("org"),
        listOf("project"),
        listOf("gist"),
        listOf("search"),
        listOf("attestation"),
        listOf("gpg-key"),
        listOf("ssh-key"),
        listOf("config"),
        listOf("alias"),
        listOf("extension"),
        listOf("codespace"),
        listOf("discussion"),
        listOf("skill"),
        listOf("agent-task"),
        listOf("codespace", "ports"),
    )

private val GH_ALIASES = mapOf("co" to listOf("pr", "checkout"))

fun namesCommand(word: String) =
    word == "api" ||
        word in GH_ALIASES ||
        (READ_GH_COMMANDS + BARE_READ_GH_COMMANDS).any { it[0] == word }

fun checkSubcommand(args: List<Token>) = SubcommandReader(args).read()

private class SubcommandReader(
    private val args: List<Token>,
) {
    private val rest = args.iterator()
    private var path = listOf<String>()
    private var settled = false
    private var verdict: Verdict? = null

    fun read(): Verdict? {
        while (!settled && path.size < SUBCOMMAND_DEPTH && rest.hasNext()) visit(rest.next())
        return if (settled) verdict else unlisted(path)
    }

    private fun visit(token: Token) {
        val value = token.value
        when {
            !literal(token) -> settle(undeterminable(token))
            takesNextWord(value) -> skipValue()
            value != "--" && (value.isEmpty() || value.startsWith('-')) -> Unit
            else -> extend(value)
        }
    }

    private fun skipValue() {
        val skipped = if (rest.hasNext()) rest.next() else null
        if (skipped != null && shape(skipped.raw) == Shape.SPLITTING) settle(undeterminable(skipped))
    }

    private fun extend(value: String) {
        path = (path + value).let { extended -> extended.singleOrNull()?.let { GH_ALIASES[it] } ?: extended }
        val command = exceptionalCommand(path)
        when {
            command != null -> settle(checkExceptional(command, args))
            path == AUTH_STATUS -> settle(checkAuthStatus())
            path in READ_GH_COMMANDS -> settle(null)
        }
    }

    private fun settle(found: Verdict?) {
        settled = true
        verdict = found
    }

    private fun checkAuthStatus() =
        rest.asSequence().find { !literal(it) || showsToken(it.value) }?.let { given ->
            Verdict.Deny(
                "`gh auth status` was given ${shown(given)}, which could print the credential gh stored, and with " +
                    "that credential any write can be made past this hook. Run `gh auth status` without it.",
            )
        }
}

private fun showsToken(flag: String) =
    if (flag.startsWith("--")) {
        flag.removePrefix("--").startsWith("show-token")
    } else {
        flag.startsWith('-') && 't' in flag
    }

private fun takesNextWord(flag: String) =
    when {
        '=' in flag || flag in GH_FLAGS_WITHOUT_VALUE -> false
        flag.startsWith("--") -> flag.length > 2
        else -> flag.startsWith('-') && flag.length == 2 && flag[1].code < ASCII_LIMIT
    }

private const val ASCII_LIMIT = 0x80

private fun undeterminable(token: Token) =
    Verdict.Deny(
        "${shown(token)} comes before the end of the gh subcommand and contains a shell expansion or pattern, so " +
            "which subcommand gh would run cannot be determined. Write the subcommand out literally, and put the " +
            "flags after it.",
    )

private fun unlisted(path: List<String>): Verdict? {
    if (path.isEmpty() || path in BARE_READ_GH_COMMANDS) return null
    val group = path.take(2)
    val named = (if (group in BARE_READ_GH_COMMANDS) path else group).joinToString(" ")
    return Verdict.Deny(
        "`gh $named` is not on this hook's allow-list of gh subcommands that only read from GitHub, so it cannot " +
            "be shown to leave GitHub unchanged. Read the data with a gh command that is on the list, or ask the " +
            "user to run this one. If `gh $named` really only reads, the allow-list in claude-gh-admission-hook " +
            "is what needs the entry.",
    )
}
