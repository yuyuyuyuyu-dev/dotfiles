package dev.yuyuyuyuyu.claudeghadmissionhook

const val SUBST_PLACEHOLDER = "__SUBST__"
const val BLANKS = " \t\r"
const val SEPARATORS = "();|&\n"
const val REDIRECTORS = "<>"

data class Token(
    val value: String,
    val raw: String,
    val separator: Boolean,
)

class Unanalyzable(
    message: String,
) : Exception(message)

class Heredoc(
    val body: String,
    val expands: Boolean,
)

class Words(
    val tokens: List<Token>,
    val targets: List<String>,
)

enum class Quote { SINGLE, DOUBLE }

fun quoteOf(char: Char) = if (char == '\'') Quote.SINGLE else Quote.DOUBLE

fun hasExpansion(raw: String) = '$' in raw || '`' in raw

fun expands(text: String) = hasExpansion(text) || SUBST_PLACEHOLDER in text

fun splits(token: Token) = shape(token.raw) == Shape.SPLITTING

fun literal(token: Token) = shape(token.raw) == Shape.LITERAL

fun mayBeFlag(token: Token): Boolean {
    val value = token.value
    return when {
        literal(token) -> false
        value.startsWith('-') -> expands(value.substringBefore('='))
        else -> value.startsWith('$') || value.startsWith('`') || value.startsWith(SUBST_PLACEHOLDER)
    }
}

fun unseen() = Token(SUBST_PLACEHOLDER, SUBST_PLACEHOLDER, separator = false)

fun shown(token: Token) = token.raw.replace(SUBST_PLACEHOLDER, "$(...)")

fun segments(tokens: List<Token>): List<List<Token>> {
    val found = mutableListOf<List<Token>>()
    var start = 0

    for ((index, token) in tokens.withIndex()) {
        if (token.separator) {
            if (index > start) found += tokens.subList(start, index)
            start = index + 1
        }
    }
    if (start < tokens.size) found += tokens.subList(start, tokens.size)

    return found
}

fun basename(path: String) = path.substringAfterLast('/')
