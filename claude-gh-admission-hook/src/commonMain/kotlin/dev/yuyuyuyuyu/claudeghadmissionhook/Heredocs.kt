package dev.yuyuyuyuyu.claudeghadmissionhook

private val OPENERS = listOf("do", "then", "else", "elif", "{", "!")
private const val ARITHMETIC_SUBSTITUTION = "$(("
private const val HERE_STRING = "<<<"

fun stripHeredocs(text: String) = HeredocStripper(Cursor(text)).strip()

private class Cursor(
    val text: String,
) {
    val out = StringBuilder()
    var index = 0

    val next get() = text.getOrNull(index + 1)
    val startsWord get() = out.lastOrNull().let { it == null || it.isUnicodeWhiteSpace() || it in SEPARATORS }

    fun take(count: Int) = takeUntil(minOf(index + count, text.length))

    fun takeUntil(end: Int) {
        out.append(text, index, end)
        index = end
    }
}

private class Level(
    val backticks: Boolean = false,
    val start: Int = 0,
) {
    var quote: Quote? = null
    var parens = 0
    var cases = 0
}

private class PendingHeredocs(
    private val cursor: Cursor,
) {
    val bodies = mutableListOf<Heredoc>()
    private val pending = mutableListOf<Pair<String, Boolean>>()

    val waiting get() = pending.isNotEmpty()

    fun redirect(): Boolean {
        if (cursor.text.startsWith(HERE_STRING, cursor.index)) {
            cursor.take(HERE_STRING.length)
            return true
        }
        return heredoc()
    }

    private fun heredoc(): Boolean {
        val text = cursor.text
        var end = cursor.index + 2
        if (text.getOrNull(end) == '-') end += 1
        while (text.getOrNull(end) == ' ' || text.getOrNull(end) == '\t') end += 1
        val delimiter = readWord(text, end)?.takeIf { it.value.isNotEmpty() } ?: return false
        pending += delimiter.value to delimiter.raw.none { it == '\'' || it == '"' || it == '\\' }
        cursor.takeUntil(delimiter.end)
        return true
    }

    fun readBodies(): Boolean {
        cursor.take(1)
        for ((delimiter, expands) in pending) {
            val (found, end) = body(cursor.text, cursor.index, delimiter) ?: break
            bodies += Heredoc(found, expands)
            cursor.index = end
        }
        pending.clear()
        return true
    }
}

private class HeredocStripper(
    private val cursor: Cursor,
) {
    private val text = cursor.text
    private val heredocs = PendingHeredocs(cursor)
    private val levels = mutableListOf(Level())

    private val level get() = levels.last()

    fun strip(): Pair<String, List<Heredoc>> {
        while (cursor.index < text.length) visit(text[cursor.index])
        val balanced = levels.size == 1 && levels.none { it.quote != null }
        return if (balanced) cursor.out.toString() to heredocs.bodies else text to emptyList()
    }

    private fun visit(char: Char) {
        val consumed =
            when {
                level.quote == Quote.SINGLE -> closesQuote(char, '\'')
                char == '\\' -> cursor.take(2).let { true }
                char == '$' && cursor.next == '(' -> openSubstitution()
                char == '`' -> backtick()
                level.quote == Quote.DOUBLE -> closesQuote(char, '"')
                else -> unquoted(char)
            }
        if (!consumed) cursor.take(1)
    }

    private fun closesQuote(
        char: Char,
        mark: Char,
    ): Boolean {
        if (char == mark) level.quote = null
        return false
    }

    private fun openSubstitution(): Boolean {
        val opensArithmetic = text.startsWith(ARITHMETIC_SUBSTITUTION, cursor.index)
        if (!opensArithmetic || !arithmetic(cursor.index + ARITHMETIC_SUBSTITUTION.length)) {
            cursor.take(2)
            levels += Level(start = cursor.out.length)
        }
        return true
    }

    private fun backtick(): Boolean {
        if (level.backticks) {
            levels.removeAt(levels.lastIndex)
        } else {
            levels += Level(backticks = true, start = cursor.out.length + 1)
        }
        return false
    }

    private fun unquoted(char: Char): Boolean =
        when (char) {
            '\'', '"' -> {
                level.quote = quoteOf(char)
                false
            }
            '(' -> openParenthesis()
            'c', 'e' -> {
                countCases(char)
                false
            }
            ')' -> {
                closeParenthesis()
                false
            }
            '#' -> cursor.startsWord && comment()
            '<' -> cursor.next == '<' && heredocs.redirect()
            '\n' -> heredocs.waiting && heredocs.readBodies()
            else -> false
        }

    private fun arithmetic(from: Int): Boolean {
        val end = closing(text, from) ?: return false
        cursor.takeUntil(end)
        return true
    }

    private fun openParenthesis(): Boolean {
        if (cursor.next == '(' && cursor.startsWord) return arithmetic(cursor.index + 2)
        if (levels.size > 1) level.parens += 1
        return false
    }

    private fun closeParenthesis() {
        if (levels.size == 1 || level.backticks) return
        if (level.parens > 0) {
            level.parens -= 1
        } else if (level.cases == 0) {
            levels.removeAt(levels.lastIndex)
        }
    }

    private fun countCases(char: Char) {
        if (!cursor.startsWord) return
        val index = cursor.index
        if (char == 'c' && keyword(text, index, "case") && commandsNext(cursor.out, level.start)) level.cases += 1
        if (char == 'e' && keyword(text, index, "esac")) level.cases = maxOf(0, level.cases - 1)
    }

    private fun comment(): Boolean {
        cursor.takeUntil(text.indexOf('\n', cursor.index).takeIf { it >= 0 } ?: text.length)
        return true
    }
}

private fun keyword(
    text: String,
    at: Int,
    name: String,
): Boolean {
    val after = text.getOrNull(at + name.length)
    return text.regionMatches(at, name, 0, name.length) && endsKeyword(after)
}

private fun endsKeyword(after: Char?) = after == null || after.isUnicodeWhiteSpace() || after == ';' || after == ')'

private fun commandsNext(
    out: StringBuilder,
    start: Int,
): Boolean {
    val before = (if (start <= out.length) out.substring(start) else "").trimEnd(' ', '\t')
    val last = before.lastOrNull() ?: return true
    return last in SEPARATORS || before.substring(before.indexOfLast { it.isUnicodeWhiteSpace() } + 1) in OPENERS
}

private fun closing(
    text: String,
    from: Int,
): Int? {
    val newline = text.indexOf('\n', from).takeIf { it >= 0 } ?: text.length
    val found = text.indexOf("))", from)
    return if (found in 0 until newline) found + 2 else null
}

private fun body(
    text: String,
    from: Int,
    delimiter: String,
): Pair<String, Int>? {
    val lines = mutableListOf<String>()
    var start = from
    var found: Pair<String, Int>? = null
    while (found == null && start < text.length) {
        val end = text.indexOf('\n', start).takeIf { it >= 0 } ?: text.length
        val line = text.substring(start, end)
        if (line.trimWhiteSpace() == delimiter) found = lines.joinToString("\n") to minOf(end + 1, text.length)
        lines += line
        start = end + 1
    }
    return found
}
