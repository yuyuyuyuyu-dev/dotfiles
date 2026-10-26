package dev.yuyuyuyuyu.claudeghadmissionhook

private val READ_METHODS = listOf("GET", "HEAD")

private val VALUE_FLAGS =
    listOf(
        "-X",
        "--method",
        "-f",
        "--raw-field",
        "-F",
        "--field",
        "-H",
        "--header",
        "--hostname",
        "--input",
        "-q",
        "--jq",
        "-t",
        "--template",
        "--cache",
    )
private val BOOL_FLAGS =
    listOf(
        "-i",
        "--include",
        "--paginate",
        "--silent",
        "--slurp",
        "--verbose",
        "-h",
        "--help",
    )
private val FIELD_FLAGS = listOf("-f", "--raw-field", "-F", "--field")

private val MUTATION = Regex("""(?i)\bmutation\b""")
private val SCHEME = Regex("""^[A-Za-z][A-Za-z0-9+.\-]*://[^/]*""")
private val API_PREFIX = Regex("""^api/(v3/)?""")

private class Api {
    var method: String? = null
    var methodRaw: String? = null
    var endpoint: String? = null
    var bodyParams = false
    val queries = mutableListOf<String>()
    var opaqueQuery = false
    var expandedQuery = false
    var flagLike: String? = null

    fun method(
        value: String,
        raw: String,
    ) {
        method = value
        methodRaw = raw
    }

    fun field(
        pair: String,
        token: Token,
    ) {
        bodyParams = true
        val key = pair.substringBefore('=')
        val value = if ('=' in pair) pair.substringAfter('=') else ""
        when {
            expandsKey(key, token) -> expandedQuery = true
            key.trimWhiteSpace() != "query" -> Unit
            value.startsWith('@') -> opaqueQuery = true
            else -> queries += value
        }
    }

    fun input() {
        bodyParams = true
        opaqueQuery = true
    }

    fun positional(
        value: String,
        token: Token,
    ) {
        if (mayBeFlagLike(value, token)) flagLike = flagLike ?: shown(token)
        if (endpoint == null) endpoint = value
    }

    private fun expandsKey(
        key: String,
        token: Token,
    ) = !literal(token) && (hasExpansion(key) || SUBST_PLACEHOLDER in key || key.trimWhiteSpace() == "query")

    private fun mayBeFlagLike(
        value: String,
        token: Token,
    ): Boolean {
        val first = value.firstOrNull()
        val plain = first != null && (first in 'A'..'Z' || first in 'a'..'z' || first in '0'..'9' || first == '/')
        return !literal(token) && expands(value) && !plain
    }
}

private class ApiReader(
    private val args: List<Token>,
) {
    private val api = Api()
    private var index = 0
    private var ended = false

    fun read(): Api {
        while (!ended && index < args.size) visit(args[index])
        return api
    }

    private fun visit(token: Token) {
        val value = token.value
        when {
            value == "--" -> endFlags()
            value.startsWith("--") && '=' in value -> inline(value, token)
            value in VALUE_FLAGS -> separate(value)
            value in BOOL_FLAGS -> index += 1
            else -> other(value, token)
        }
    }

    private fun endFlags() {
        if (api.endpoint == null) api.endpoint = args.getOrNull(index + 1)?.value
        ended = true
    }

    private fun inline(
        flag: String,
        token: Token,
    ) {
        val name = flag.substringBefore('=')
        val value = flag.substringAfter('=')
        when {
            name == "--method" -> api.method(value, token.raw)
            name in FIELD_FLAGS -> api.field(value, token)
            name == "--input" -> api.input()
            name !in VALUE_FLAGS && name !in BOOL_FLAGS -> throw Unanalyzable("unknown flag $name")
        }
        index += 1
    }

    private fun separate(flag: String) {
        val next = args.getOrNull(index + 1) ?: throw Unanalyzable("flag without value: $flag")
        when {
            flag == "-X" || flag == "--method" -> api.method(next.value, next.raw)
            flag in FIELD_FLAGS -> api.field(next.value, next)
            flag == "--input" -> api.input()
        }
        index += 2
    }

    private fun other(
        value: String,
        token: Token,
    ) {
        when {
            value.startsWith("-X") && value.length > 2 -> api.method(value.substring(2), token.raw)
            value.startsWith("-f") && value.length > 2 -> api.field(value.substring(2), token)
            value.startsWith("-F") && value.length > 2 -> api.field(value.substring(2), token)
            value.startsWith("-H") && value.length > 2 -> Unit
            value.startsWith('-') -> throw Unanalyzable("unknown flag $value")
            else -> api.positional(value, token)
        }
        index += 1
    }
}

private fun normalizeEndpoint(endpoint: String): String {
    val path =
        SCHEME
            .replaceFirst(endpoint.trimWhiteSpace(), "")
            .substringBefore('?')
            .substringBefore('#')
            .trimStart('/')
    return API_PREFIX.replaceFirst(path, "").trimEnd('/').lowercase()
}

fun checkApi(args: List<Token>): Verdict? {
    val splitting = args.find { shape(it.raw) == Shape.SPLITTING }
    return if (splitting == null) {
        judgeApi(ApiReader(args).read())
    } else {
        Verdict.Deny(
            "An argument of `gh api` contains an unquoted shell expansion or pattern (${shown(splitting)}), which " +
                "can turn into further arguments, so the request that would actually be sent cannot be determined. " +
                "Quote it, or write the request out with literal values.",
        )
    }
}

private fun judgeApi(api: Api): Verdict? {
    val flagLike = api.flagLike
    val endpoint = api.endpoint
    return when {
        flagLike != null -> {
            Verdict.Deny(
                "An argument of `gh api` ($flagLike) comes out of a shell expansion and is not the value of a " +
                    "flag, so it could turn out to be a flag that changes the request. Start it with literal " +
                    "text, or write it out literally.",
            )
        }

        endpoint == null -> {
            throw Unanalyzable("no endpoint")
        }

        normalizeEndpoint(endpoint) == "graphql" -> {
            judgeGraphql(api)
        }

        else -> {
            judgeMethod(api)
        }
    }
}

private fun judgeGraphql(api: Api) =
    when {
        api.opaqueQuery -> {
            Verdict.Deny(
                "The query body of `gh api graphql` is read from a file or stdin, so it cannot be shown to be " +
                    "free of mutations. Pass the query inline, as -f query='query { ... }', and it can be checked.",
            )
        }

        api.expandedQuery -> {
            Verdict.Deny(
                "A field of `gh api graphql` comes out of a shell expansion and could be the query, so the query " +
                    "cannot be shown to be free of mutations. Write the query out literally, and pass what varies " +
                    "as a GraphQL variable with -F.",
            )
        }

        api.queries.isEmpty() -> {
            throw Unanalyzable("graphql without inline query")
        }

        api.queries.any { MUTATION.containsMatchIn(it) } -> {
            Verdict.Deny(
                "The query passed to `gh api graphql` contains a mutation, which changes GitHub state. Send a " +
                    "query instead, or ask the user to run the mutation.",
            )
        }

        else -> {
            null
        }
    }

private fun judgeMethod(api: Api): Verdict? {
    val raw = api.methodRaw
    val effective = api.method?.uppercase() ?: if (api.bodyParams) "POST" else "GET"
    return when {
        raw != null && hasExpansion(raw) -> {
            Verdict.Deny(
                "The method given to `gh api` is a shell expansion ($raw), so it cannot be shown to be GET or " +
                    "HEAD. Write the method out literally.",
            )
        }

        effective in READ_METHODS -> {
            null
        }

        api.method == null -> {
            Verdict.Deny(
                "`gh api` was given -f/-F/--input with no method, and gh sends POST rather than GET as soon as it " +
                    "has fields, so this would write. Add --method GET if the fields are query parameters of a read.",
            )
        }

        else -> {
            Verdict.Deny(
                "`gh api` would send $effective, which is not a read and can change GitHub state. Only GET and " +
                    "HEAD are allowed here.",
            )
        }
    }
}
