package dev.yuyuyuyuyu.claudeghadmissionhook

private const val MAX_DEPTH = 3

private val CONTINUATION = Regex("""\\\r?\n""")

private val GH_INVOCATION = Regex("""(^|[^A-Za-z0-9_])([^$WHITE_SPACE;|&()]*/)?gh[$WHITE_SPACE]""")
private val GIT_TAG_INVOCATION =
    Regex(
        """(^|[^A-Za-z0-9_])([^$WHITE_SPACE;|&()]*/)?git[$WHITE_SPACE][^;|&]*?""" +
            """(\btag\b|--tags\b|--follow-tags\b|--mirror\b|refs/tags/)""",
    )
private val HTTP_CLIENT_INVOCATION = Regex("""(^|[^A-Za-z0-9_])([^$WHITE_SPACE;|&()]*/)?(curl|wget)\b""")

fun mentionsGitHub(text: String) =
    GH_INVOCATION.containsMatchIn(text) ||
        GIT_TAG_INVOCATION.containsMatchIn(text) ||
        (
            HTTP_CLIENT_INVOCATION.containsMatchIn(text) &&
                (GH_API_URL.containsMatchIn(text) || GH_CREDENTIAL.containsMatchIn(text))
        )

class Script(
    val text: String,
    val heredocs: List<Heredoc>,
    val githubVariables: List<String>,
    val depth: Int,
)

class Launch(
    val command: List<Token>,
    val fed: Boolean,
    val direct: Boolean,
)

fun checkCommand(
    command: String,
    depth: Int,
): List<Verdict> = if (depth > MAX_DEPTH) checkTooDeep(command) else checkScript(command, depth)

private fun checkTooDeep(command: String) =
    if (mentionsGitHub(command)) {
        listOf(
            Verdict.Deny(
                "This command mentions gh, a git tag or the GitHub API inside more nested shells and " +
                    "substitutions than this hook reads, so what it would run cannot be determined. " +
                    "Run the GitHub part as a command of its own.",
            ),
        )
    } else {
        emptyList()
    }

private fun checkScript(
    command: String,
    depth: Int,
): List<Verdict> {
    val (stripped, heredocs) = stripHeredocs(CONTINUATION.replace(command, " "))
    val githubVariables = githubVariables(stripped)
    val (text, inners) = extractSubstitutions(stripped)
    val verdicts = mutableListOf<Verdict>()

    for (inner in inners) verdicts += checkCommand(inner, depth + 1)
    for (heredoc in heredocs.filter { it.expands }) {
        for (inner in extractSubstitutions(heredoc.body).second) verdicts += checkCommand(inner, depth + 1)
    }

    val script = Script(text, heredocs, githubVariables, depth)
    val words = tokenize(text, comments = true)
    verdicts += if (words == null) checkUntokenized(text) else checkWords(words, script) + checkUncommented(script)
    return verdicts
}

private fun checkUntokenized(text: String) =
    if (mentionsGitHub(text)) {
        listOf(
            Verdict.Deny(
                "This command mentions gh, a git tag or the GitHub API, but it could not be tokenized -- " +
                    "an unbalanced quote is the usual cause -- so what it would run cannot be determined. " +
                    "It is denied rather than guessed at. Run the GitHub part as a command of its own.",
            ),
        )
    } else {
        emptyList()
    }

private fun checkUncommented(script: Script): List<Verdict> {
    val words = if ('#' in script.text) tokenize(script.text, comments = false) else null
    return words?.let { checkWords(it, script) }.orEmpty().map { verdict ->
        if (verdict is Verdict.Deny) {
            Verdict.Deny(
                "This command has a # in it, and when the text after the # is read as arguments rather " +
                    "than as a comment, the command is one this hook denies. Run it without the comment.",
            )
        } else {
            verdict
        }
    }
}
