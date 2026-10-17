package dev.yuyuyuyuyu.claudeghadmissionhook

private val READ_METHODS = listOf("GET", "HEAD")

val HTTP_CLIENTS = listOf("curl", "wget")

val GH_API_URL = Regex("""(?i)\b(?:api|uploads)\.github\.com\b|/api/v3/|/api/graphql\b""")
val GH_CREDENTIAL = Regex("""(?i)gh[$WHITE_SPACE]+auth[$WHITE_SPACE]+token|GH_TOKEN|GITHUB_TOKEN|GH_ENTERPRISE_TOKEN""")
private val ASSIGNMENT_VALUE =
    Regex(
        """(?:^|[;&|(]|[$WHITE_SPACE])([A-Za-z_][A-Za-z0-9_]*)=""" +
            """((?:\$\([^)]*\)|"[^"]*"|'[^']*'|[^$WHITE_SPACE;&|]*))""",
    )

fun githubVariables(text: String) =
    ASSIGNMENT_VALUE
        .findAll(text)
        .filter { GH_API_URL.containsMatchIn(it.groupValues[2]) || GH_CREDENTIAL.containsMatchIn(it.groupValues[2]) }
        .map { it.groupValues[1] }
        .distinct()
        .toList()

fun checkHttp(
    name: String,
    args: List<Token>,
    githubVariables: List<String>,
): Verdict? {
    val request = Request(name, githubVariables.map { Regex("""\$\{?""" + Regex.escape(it) + """\b""") })
    args.forEach { request.read(it) }
    return if (request.targetsGitHub) request.judge(args) else null
}

private class Request(
    private val name: String,
    private val variables: List<Regex>,
) {
    private val client = if (name == "wget") WGET else CURL
    private var method: String? = null
    private var methodRaw: String? = null
    private var head = false
    private var forceGet = false
    private var upload = false
    private var body = false
    private var splitting: String? = null
    private var flagLike: String? = null
    private var options: String? = null
    private var unknown: String? = null
    private var takeMethodNext = false
    private var valueExpected = false
    var targetsGitHub = false
        private set

    fun read(token: Token) {
        note(token)
        if (takeMethodNext) {
            method = token.value
            methodRaw = token.raw
            takeMethodNext = false
        } else {
            interpret(token)
        }
    }

    private fun note(token: Token) {
        val value = token.value
        val free = !valueExpected
        if (splits(token)) {
            splitting = splitting ?: shown(token)
        } else if (free && mayBeFlag(token)) {
            flagLike = flagLike ?: shown(token)
        }
        if (free && value.startsWith('-') && !client.knows(value)) unknown = unknown ?: shown(token)
        if (free && client.readsOptions(value)) options = options ?: shown(token)
        valueExpected = free && client.takesNextWord(value)
    }

    private fun interpret(token: Token) {
        val value = token.value
        if (mentionsGitHub(value)) targetsGitHub = true
        when {
            value in METHOD_FLAGS -> takeMethodNext = true
            setsMethod(value) -> setMethod(value.substringAfter('='), token.raw)
            value == "-I" || value == "--head" -> head = true
            value == "-G" || value == "--get" -> forceGet = true
            value in UPLOAD_FLAGS -> upload = true
            sendsBody(value) -> body = true
            isShortBundle(value) -> takeMethodNext = readShortBundle(value.substring(1), token.raw)
        }
    }

    private fun mentionsGitHub(value: String) =
        GH_API_URL.containsMatchIn(value) ||
            GH_CREDENTIAL.containsMatchIn(value) ||
            variables.any { it.containsMatchIn(value) }

    private fun setMethod(
        value: String,
        raw: String,
    ) {
        method = value
        methodRaw = raw
    }

    private fun readShortBundle(
        letters: String,
        raw: String,
    ): Boolean {
        val stop = letters.indexOfFirst { it in "XTdF" }.let { if (it < 0) letters.length else it }
        val before = letters.take(stop)
        if ('I' in before) head = true
        if ('G' in before) forceGet = true
        val rest = letters.substring(minOf(stop + 1, letters.length))
        return when (letters.getOrNull(stop)) {
            'X' -> rest.isEmpty().also { if (!it) setMethod(rest, raw) }
            'T' -> false.also { upload = true }
            'd', 'F' -> false.also { body = true }
            else -> false
        }
    }

    fun judge(args: List<Token>): Verdict? = unreadable() ?: judgeMethod(args)

    private fun unreadable(): Verdict? {
        val raw = methodRaw
        return when {
            splitting != null ->
                Verdict.Deny(
                    "An argument of `$name` contains an unquoted shell expansion or pattern ($splitting), which " +
                        "can turn into further arguments, so the request that would be sent to the GitHub API " +
                        "cannot be determined. Quote it, or write the request out with literal values.",
                )
            options != null ->
                Verdict.Deny(
                    "`$name` was given $options, which makes it read further options from somewhere this hook " +
                        "cannot see, so the request that would be sent to the GitHub API cannot be determined. " +
                        "Write the options out on the command line.",
                )
            flagLike != null ->
                Verdict.Deny(
                    "An argument of `$name` ($flagLike) comes out of a shell expansion and is not the value of a " +
                        "flag this hook knows, so it could turn out to be a flag that changes the request sent to " +
                        "the GitHub API. Start it with literal text, or write it out literally.",
                )
            raw != null && hasExpansion(raw) ->
                Verdict.Deny(
                    "The HTTP method given to `$name` is a shell expansion ($raw), so it cannot be shown to be GET " +
                        "or HEAD. Write the method out literally.",
                )
            else -> null
        }
    }

    private fun judgeMethod(args: List<Token>): Verdict? {
        val effective =
            method?.uppercase() ?: when {
                head -> "HEAD"
                forceGet -> "GET"
                upload -> "PUT"
                body -> "POST"
                else -> "GET"
            }
        return if (effective in READ_METHODS) {
            unknownFlag() ?: unseenOptions(name, args)
        } else {
            Verdict.Deny(
                "`$name` would send $effective to the GitHub API, which can change repository state. Only GET and " +
                    "HEAD are allowed here. Note that a body flag such as -d, -F or -T sets the method even when -X " +
                    "is absent.",
            )
        }
    }

    private fun unknownFlag() =
        unknown?.let { flag ->
            Verdict.Deny(
                "`$name` was given $flag, a flag this hook does not know, so the request that would be sent to the " +
                    "GitHub API cannot be determined. If $flag leaves the request a read, the flag table in " +
                    "claude-gh-admission-hook is what needs the entry.",
            )
        }
}

private fun setsMethod(value: String) = value.startsWith("--request=") || value.startsWith("--method=")

private fun sendsBody(value: String) = value.substringBefore('=') in BODY_FLAGS

private fun isShortBundle(value: String) = value.length > 1 && value.startsWith('-') && !value.startsWith("--")

private fun unseenOptions(
    name: String,
    args: List<Token>,
) = when {
    name == "wget" && args.any { it.raw == "--no-config" } -> null
    name == "wget" ->
        Verdict.Deny(
            "`wget` reads further options from its wgetrc file unless --no-config is given, and those options " +
                "could change the request sent to the GitHub API. Add --no-config.",
        )
    args.firstOrNull()?.raw in CURL_WITHOUT_FILE -> null
    else ->
        Verdict.Deny(
            "`curl` reads further options from its .curlrc file unless -q is its first argument, and those options " +
                "could change the request sent to the GitHub API. Put -q first, as in `curl -q URL`.",
        )
}
