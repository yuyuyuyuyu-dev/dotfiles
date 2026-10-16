package dev.yuyuyuyuyu.claudeghadmissionhook

private const val HEXADECIMAL = 16
private const val ESCAPE_DIGITS = 4

private val WHITE_SPACE_CHARACTERS =
    "\t\n\u000B\u000C\r \u0085  " + (' '..' ').joinToString("") + "    　"

val WHITE_SPACE =
    WHITE_SPACE_CHARACTERS.map { "\\u" + it.code.toString(HEXADECIMAL).padStart(ESCAPE_DIGITS, '0') }.joinToString("")

fun Char.isUnicodeWhiteSpace() = this in WHITE_SPACE_CHARACTERS

fun String.trimWhiteSpace() = trim { it.isUnicodeWhiteSpace() }
