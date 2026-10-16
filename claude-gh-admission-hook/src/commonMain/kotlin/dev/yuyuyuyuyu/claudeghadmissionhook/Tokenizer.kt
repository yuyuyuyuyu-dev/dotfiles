package dev.yuyuyuyuyu.claudeghadmissionhook

private const val REDIRECTION = "<>&|!"

fun tokenize(
    text: String,
    comments: Boolean,
) = Tokenizer(text, comments).tokenize()

private class Tokenizer(
    private val text: String,
    private val comments: Boolean,
) {
    private val tokens = mutableListOf<Token>()
    private val targets = mutableListOf<String>()
    private var index = 0
    private var failed = false

    fun tokenize(): Words? {
        while (!failed && index < text.length) visit(text[index])
        return if (failed) null else Words(tokens, targets)
    }

    private fun visit(char: Char) {
        when {
            char in BLANKS -> index += 1
            comments && char == '#' -> skipComment()
            char in REDIRECTORS || joinsOutputs(index) -> redirection()
            char in SEPARATORS -> separator()
            else -> argument()
        }
    }

    private fun skipComment() {
        index = text.indexOf('\n', index).takeIf { it >= 0 } ?: text.length
    }

    private fun redirection() {
        while (index < text.length && text[index] in REDIRECTION) index += 1
        while (index < text.length && text[index] in BLANKS) index += 1
        val target = readWord(text, index)
        if (target == null) {
            failed = true
        } else {
            targets += target.value
            index = target.end
        }
    }

    private fun separator() {
        val start = index
        index += 1
        while (continuesSeparator()) index += 1
        val run = text.substring(start, index)
        tokens += Token(run, run, separator = true)
    }

    private fun continuesSeparator(): Boolean {
        val char = text.getOrNull(index)
        return char != null && char in SEPARATORS && (text[index - 1] == '&' || !joinsOutputs(index))
    }

    private fun argument() {
        val found = readWord(text, index)
        if (found == null) {
            failed = true
            return
        }
        index = found.end
        val descriptor =
            text.getOrNull(index)?.let { it in REDIRECTORS } == true &&
                found.raw.isNotEmpty() &&
                found.raw.all { it in '0'..'9' }
        if (!descriptor) tokens += Token(found.value, found.raw, separator = false)
    }

    private fun joinsOutputs(at: Int) = text.getOrNull(at) == '&' && text.getOrNull(at + 1) == '>'
}
