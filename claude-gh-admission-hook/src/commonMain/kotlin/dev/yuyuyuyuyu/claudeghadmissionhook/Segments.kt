package dev.yuyuyuyuyu.claudeghadmissionhook

private val KEYWORDS =
    listOf(
        "if",
        "then",
        "else",
        "elif",
        "fi",
        "while",
        "until",
        "do",
        "done",
        "for",
        "case",
        "esac",
        "in",
        "select",
        "function",
        "!",
        "{",
        "}",
    )
private val WRAPPERS =
    listOf(
        "command",
        "builtin",
        "exec",
        "env",
        "nohup",
        "time",
        "nice",
        "stdbuf",
        "sudo",
        "doas",
        "su",
        "noglob",
        "nocorrect",
        "coproc",
        "setsid",
        "caffeinate",
        "unbuffer",
        "arch",
        "chronic",
    )
private val RUNNERS = listOf("xargs", "timeout", "watch", "parallel", "ionice", "flock", "retry")
private val FEEDERS = listOf("xargs", "parallel")
private val MOVERS = listOf("cd", "pushd", "popd")

private val ASSIGNMENT = Regex("""^[A-Za-z_][A-Za-z0-9_]*=""")
private val GIT_FILES = Regex("""(^|[^A-Za-z0-9_-])\.git(/|\z|[^A-Za-z0-9_-])|gitconfig|git/config""")
private val GIT_VARIABLE = Regex("""^GIT_[A-Za-z0-9_]*=""")

fun commandName(token: Token) = basename(token.value).removePrefix("=")

private fun launches(head: Token?) = head != null && commandName(head).let { it in WRAPPERS || it in RUNNERS }

private fun commands(given: List<Token>): List<Launch> {
    val segment = given.dropWhile { ASSIGNMENT.containsMatchIn(it.value) || it.value in KEYWORDS }
    val launches = launches(segment.firstOrNull())
    val found = mutableListOf<Launch>()
    if (!launches) found += Launch(segment, fed = false, direct = true)

    for (index in 1 until segment.size) {
        if (launched(segment, index, launches)) {
            found +=
                Launch(
                    segment.subList(index, segment.size),
                    fed = segment.subList(0, index).any { basename(it.value) in FEEDERS },
                    direct = launches,
                )
        }
    }
    return found
}

private fun launched(
    segment: List<Token>,
    index: Int,
    launches: Boolean,
) = when (val name = commandName(segment[index])) {
    "git" -> true
    "eval" -> launches
    "gh" -> launches || segment.getOrNull(index + 1)?.let { namesCommand(it.value) } == true
    else -> name in SHELLS || name in HTTP_CLIENTS
}

private fun unsettles(segment: List<Token>): String? {
    if (segment.any { GIT_VARIABLE.containsMatchIn(it.value) }) return "sets a GIT_ environment variable"
    val head =
        segment.find { token ->
            !ASSIGNMENT.containsMatchIn(token.value) &&
                token.value !in KEYWORDS &&
                basename(token.value) !in WRAPPERS &&
                !token.value.startsWith('-')
        }
    return if (head != null && head.value in MOVERS) "changes the directory" else null
}

private fun scripts(segment: List<Token>): List<Token> {
    val launches = launches(segment.find { !ASSIGNMENT.containsMatchIn(it.value) && it.value !in KEYWORDS })
    return segment.filter { token -> launches && token.value.any { it.isUnicodeWhiteSpace() } }
}

private fun namesGitFile(word: String) = word.none { it.isUnicodeWhiteSpace() } && GIT_FILES.containsMatchIn(word)

fun checkWords(
    words: Words,
    script: Script,
): List<Verdict> {
    val found = segments(words.tokens)
    val verdicts = mutableListOf<Verdict>()
    verdicts += found.mapNotNull { unsettles(it) }.map { Verdict.Unsettling(it) }
    if (words.targets.any { namesGitFile(it) } || words.tokens.any { namesGitFile(it.value) }) {
        verdicts += Verdict.Unsettling("names the .git directory or a git configuration file")
    }
    for (given in found.flatMap { scripts(it) }) verdicts += checkCommand(given.value, script.depth + 1)
    for (launch in found.flatMap { commands(it) }) verdicts += checkLaunch(launch, script)
    return verdicts
}
