package dev.yuyuyuyuyu.claudeghadmissionhook

class Word(
    val value: String,
    val raw: String,
    val end: Int,
)

fun readWord(
    text: String,
    from: Int,
) = WordReader(text, from).read()

private class WordReader(
    private val text: String,
    private var index: Int,
) {
    private val value = StringBuilder()
    private val raw = StringBuilder()
    private var failed = false

    fun read(): Word? {
        while (!failed && index < text.length && !endsWord(text[index])) visit(text[index])
        return if (failed) null else Word(value.toString(), raw.toString(), index)
    }

    private fun endsWord(char: Char) = char in BLANKS || char in SEPARATORS || char in REDIRECTORS

    private fun visit(char: Char) {
        when (char) {
            '\'' -> singleQuoted()
            '"' -> doubleQuoted()
            '\\' -> escaped()
            else -> {
                value.append(char)
                raw.append(char)
                index += 1
            }
        }
    }

    private fun singleQuoted() {
        val close = text.indexOf('\'', index + 1)
        if (close < 0) {
            failed = true
        } else {
            value.append(text, index + 1, close)
            raw.append(text, index, close + 1)
            index = close + 1
        }
    }

    private fun doubleQuoted() {
        raw.append('"')
        index += 1
        while (index < text.length && text[index] != '"') insideDoubleQuotes(text[index])
        if (index < text.length) {
            raw.append('"')
            index += 1
        } else {
            failed = true
        }
    }

    private fun insideDoubleQuotes(inner: Char) {
        val escaped = text.getOrNull(index + 1)
        if (inner == '\\' && escaped != null) {
            raw.append(inner).append(escaped)
            if (escaped != '"' && escaped != '\\') value.append(inner)
            value.append(escaped)
            index += 2
        } else {
            raw.append(inner)
            value.append(inner)
            index += 1
        }
    }

    private fun escaped() {
        val escaped = text.getOrNull(index + 1)
        if (escaped == null) {
            failed = true
        } else {
            raw.append('\\').append(escaped)
            value.append(escaped)
            index += 2
        }
    }
}
