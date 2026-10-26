package dev.yuyuyuyuyu.claudeghadmissionhook

val SHELLS = listOf("bash", "sh", "zsh", "dash", "ksh")
private val SHELL_VALUE_FLAGS = listOf("--rcfile", "--init-file")

fun checkLaunch(
    launch: Launch,
    script: Script,
): List<Verdict> {
    val head = launch.command.firstOrNull() ?: return emptyList()
    val name = commandName(head)
    val args = launch.command.drop(1)
    return when {
        name in SHELLS -> checkShell(name, args, launch.direct, script)
        name == "eval" -> checkEval(args, script)
        else -> listOfNotNull(checkProgram(name, args, launch, script))
    }
}

private fun checkProgram(
    name: String,
    args: List<Token>,
    launch: Launch,
    script: Script,
): Verdict? {
    val check: ((List<Token>) -> Verdict?)? =
        when (name) {
            "git" -> { given -> checkGit(given, launch.direct) }
            "gh" -> ::checkGh
            in HTTP_CLIENTS -> { given -> checkHttp(name, given, script.githubVariables) }
            else -> null
        }
    return when {
        check == null -> null
        launch.fed -> checkFed(name, args, check)
        else -> check(args)
    }
}

private fun checkGh(args: List<Token>): Verdict? =
    when (args.firstOrNull()?.value) {
        null -> null
        "api" -> checkApiReadably(args.drop(1))
        else -> checkSubcommand(args)
    }

private fun checkApiReadably(args: List<Token>) =
    try {
        checkApi(args)
    } catch (reason: Unanalyzable) {
        Verdict.Deny(
            "The `gh api` invocation could not be parsed (${reason.message}), so the request it would send " +
                "cannot be determined, and a request that cannot be read cannot be shown to be a read.",
        )
    }

private fun checkFed(
    name: String,
    args: List<Token>,
    check: (List<Token>) -> Verdict?,
): Verdict? {
    val written = check(args)
    val deniedWithMore = written !is Verdict.Deny && check(args + unseen()) is Verdict.Deny
    return if (deniedWithMore) {
        Verdict.Deny(
            "This $name command is run through xargs or parallel, which adds arguments that this hook " +
                "cannot see, and they could turn it into a write. Give $name all of its arguments on the " +
                "command line itself.",
        )
    } else {
        written
    }
}

private fun checkEval(
    args: List<Token>,
    script: Script,
): List<Verdict> {
    val verdicts = mutableListOf<Verdict>()
    if (args.any { mayBeFlag(it) } && mentionsGitHub(script.text)) {
        verdicts +=
            Verdict.Deny(
                "An argument of `eval` comes out of a shell expansion, and this command mentions gh, a git " +
                    "tag or the GitHub API, so what `eval` would run cannot be determined. Write the command " +
                    "out literally.",
            )
    }
    verdicts += checkCommand(args.joinToString(" ") { it.value }, script.depth + 1)
    return verdicts
}

private sealed interface Source {
    class Inline(
        val token: Token,
    ) : Source

    object Input : Source

    object File : Source
}

private class SourceReader(
    private val args: List<Token>,
) {
    private var inline = false
    private var input = false
    private var index = 0

    fun read(): Source {
        var found: Source? = null
        while (found == null && index < args.size) found = visit(args[index])
        return found ?: Source.Input
    }

    private fun visit(token: Token): Source? {
        val value = token.value
        index += 1
        return when {
            value == "-" -> {
                Source.Input
            }

            value.startsWith("--") -> {
                if (value in SHELL_VALUE_FLAGS) index += 1
                null
            }

            value.startsWith('-') || value.startsWith('+') -> {
                option(value)
                null
            }

            inline -> {
                Source.Inline(token)
            }

            input -> {
                Source.Input
            }

            else -> {
                Source.File
            }
        }
    }

    private fun option(value: String) {
        val letters = value.substring(1)
        inline = inline || (value.startsWith('-') && 'c' in letters)
        input = input || 's' in letters
        if (letters.endsWith('o') || letters.endsWith('O')) index += 1
    }
}

private fun checkShell(
    name: String,
    args: List<Token>,
    direct: Boolean,
    script: Script,
): List<Verdict> {
    val source = SourceReader(args).read()
    return when {
        source is Source.Inline -> checkInlineScript(name, source.token, script)
        source is Source.Input && direct -> checkStandardInput(name, script)
        else -> emptyList()
    }
}

private fun checkInlineScript(
    name: String,
    given: Token,
    script: Script,
): List<Verdict> {
    val verdicts = mutableListOf<Verdict>()
    if (mayBeFlag(given) && mentionsGitHub(script.text)) {
        verdicts +=
            Verdict.Deny(
                "The script given to `$name -c` comes out of a shell expansion, and this command mentions gh, " +
                    "a git tag or the GitHub API, so what the script would run cannot be determined. Write the " +
                    "script out literally.",
            )
    }
    verdicts += checkCommand(given.value, script.depth + 1)
    return verdicts
}

private fun checkStandardInput(
    name: String,
    script: Script,
): List<Verdict> {
    val verdicts = mutableListOf<Verdict>()
    for (heredoc in script.heredocs) verdicts += checkCommand(heredoc.body, script.depth + 1)
    if (mentionsGitHub(script.text)) {
        verdicts +=
            Verdict.Deny(
                "`$name` is given its script on standard input here, and this command mentions gh, a git tag " +
                    "or the GitHub API outside a here-document, so what the script would run cannot be " +
                    "determined. Run the GitHub part as a command of its own.",
            )
    }
    return verdicts
}
