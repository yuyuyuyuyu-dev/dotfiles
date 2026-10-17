package dev.yuyuyuyuyu.claudeghadmissionhook

fun extractSubstitutions(text: String) = SubstitutionExtractor(text).extract()

private class SubstitutionExtractor(
    private val text: String,
) {
    private val out = StringBuilder()
    private val inners = mutableListOf<String>()
    private var quote: Quote? = null
    private var index = 0

    fun extract(): Pair<String, List<String>> {
        while (index < text.length) visit(text[index])
        return out.toString() to inners
    }

    private fun visit(char: Char) {
        when {
            char == '\\' && index + 1 < text.length -> copy(2)
            quote == Quote.SINGLE -> closeQuote(char, '\'')
            quote == null && (char == '\'' || char == '"') -> openQuote(char)
            quote == Quote.DOUBLE && char == '"' -> closeQuote(char, '"')
            substitutes(char) -> parenthesized()
            char == '`' -> backticked()
            else -> copy(1)
        }
    }

    private fun copy(count: Int) {
        out.append(text, index, index + count)
        index += count
    }

    private fun openQuote(char: Char) {
        quote = quoteOf(char)
        copy(1)
    }

    private fun closeQuote(
        char: Char,
        mark: Char,
    ) {
        if (char == mark) quote = null
        copy(1)
    }

    private fun substitutes(char: Char): Boolean {
        val opens = char == '$' || (quote == null && char in REDIRECTORS)
        return opens && text.getOrNull(index + 1) == '('
    }

    private fun parenthesized() {
        val close = matchingParenthesis(index + 1)
        if (close == null) {
            copy(1)
        } else {
            substitute(index + 2, close)
        }
    }

    private fun backticked() {
        val close = text.indexOf('`', index + 1)
        if (close < 0) {
            copy(1)
        } else {
            substitute(index + 1, close)
        }
    }

    private fun substitute(
        start: Int,
        close: Int,
    ) {
        inners += text.substring(start, close)
        out.append(SUBST_PLACEHOLDER)
        index = close + 1
    }

    private fun matchingParenthesis(from: Int): Int? {
        var depth = 0
        var scan = from
        var found: Int? = null
        while (found == null && scan < text.length) {
            if (text[scan] == '(') depth += 1
            if (text[scan] == ')') depth -= 1
            if (text[scan] == ')' && depth == 0) found = scan
            scan += 1
        }
        return found
    }
}
