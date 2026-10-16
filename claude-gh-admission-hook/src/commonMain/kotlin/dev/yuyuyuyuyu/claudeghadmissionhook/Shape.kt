package dev.yuyuyuyuyu.claudeghadmissionhook

private const val GLOBS = "*?["

enum class Shape { LITERAL, OPAQUE, SPLITTING }

fun shape(raw: String) = ShapeReader(raw).read()

private class ShapeReader(
    private val raw: String,
) {
    private var quote: Quote? = null
    private var escaped = false
    private var opaque = false
    private var splitting = false

    fun read(): Shape {
        var offset = 0
        while (!splitting && offset < raw.length) {
            visit(offset)
            offset += 1
        }
        return when {
            splitting -> Shape.SPLITTING
            opaque -> Shape.OPAQUE
            else -> Shape.LITERAL
        }
    }

    private fun visit(offset: Int) {
        val char = raw[offset]
        when {
            escaped -> escaped = false
            quote == Quote.SINGLE -> if (char == '\'') quote = null
            char == '\\' -> escaped = true
            quote != null -> if (char == '"') quote = null else opaque = opaque || expandsAt(offset)
            char == '\'' || char == '"' -> quote = quoteOf(char)
            else -> splitting = expandsAt(offset) || char in GLOBS || bracesAt(offset)
        }
    }

    private fun expandsAt(offset: Int): Boolean {
        val char = raw[offset]
        return char == '$' || char == '`' || raw.startsWith(SUBST_PLACEHOLDER, offset)
    }

    private fun bracesAt(offset: Int): Boolean {
        val rest = raw.substring(offset)
        return rest.startsWith('{') && (',' in rest || ".." in rest)
    }
}
