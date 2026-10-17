package dev.yuyuyuyuyu.claudeghadmissionhook

val BODY_FLAGS =
    listOf(
        "-d",
        "--data",
        "--data-raw",
        "--data-ascii",
        "--data-binary",
        "--data-urlencode",
        "--json",
        "-F",
        "--form",
        "--form-string",
        "--post-data",
        "--post-file",
        "--body-data",
        "--body-file",
    )
val UPLOAD_FLAGS = listOf("-T", "--upload-file")

val METHOD_FLAGS = listOf("-X", "--request", "--method")
private val CURL_VALUE_FLAGS =
    listOf(
        "--header",
        "--output",
        "--user",
        "--user-agent",
        "--referer",
        "--cookie",
        "--cookie-jar",
        "--write-out",
        "--max-time",
        "--connect-timeout",
        "--retry",
        "--retry-delay",
        "--retry-max-time",
        "--proxy",
        "--url",
        "--cacert",
        "--cert",
        "--key",
        "--range",
        "--resolve",
        "--dump-header",
        "--stderr",
    )
private val WGET_VALUE_FLAGS =
    listOf(
        "--header",
        "--output-document",
        "--user-agent",
        "--user",
        "--password",
        "--tries",
        "--timeout",
        "--directory-prefix",
        "--output-file",
        "--wait",
    )
private val CURL_PLAIN_FLAGS =
    listOf(
        "--silent",
        "--show-error",
        "--location",
        "--location-trusted",
        "--fail",
        "--fail-with-body",
        "--include",
        "--verbose",
        "--insecure",
        "--disable",
        "--no-buffer",
        "--remote-name",
        "--remote-header-name",
        "--remote-time",
        "--globoff",
        "--compressed",
        "--head",
        "--get",
        "--http1.1",
        "--http2",
        "--no-progress-meter",
        "--progress-bar",
        "--ipv4",
        "--ipv6",
        "--netrc",
        "--tlsv1.2",
        "--tlsv1.3",
    )
private val WGET_PLAIN_FLAGS =
    listOf(
        "--quiet",
        "--verbose",
        "--no-verbose",
        "--server-response",
        "--continue",
        "--timestamping",
        "--no-check-certificate",
        "--no-config",
        "--spider",
        "--content-on-error",
        "--no-clobber",
        "--inet4-only",
        "--inet6-only",
        "--debug",
    )
private val WGET_PLAIN_WORDS = listOf("-nv", "-nc", "-nd", "-nH")
private const val CURL_PLAIN_LETTERS = "sSLfivkqNOJg46#IGnR"
private const val WGET_PLAIN_LETTERS = "qvSc46N"
private const val CURL_VALUE_LETTERS = "HouAebcwmxrDEyYzKCXdFT"
private const val WGET_VALUE_LETTERS = "OUtTPoawQADe"

val CURL_WITHOUT_FILE = listOf("-q", "--disable")
private val CURL_OPTION_FILES = listOf("--config", "-K")
private val WGET_OPTION_FILES = listOf("--config", "--execute", "-e")

class Client(
    val valueFlags: List<String>,
    val plainFlags: List<String>,
    val plainWords: List<String>,
    val optionFiles: List<String>,
    val plainLetters: String,
    val valueLetters: String,
)

val CURL =
    Client(
        valueFlags = CURL_VALUE_FLAGS,
        plainFlags = CURL_PLAIN_FLAGS,
        plainWords = emptyList(),
        optionFiles = CURL_OPTION_FILES,
        plainLetters = CURL_PLAIN_LETTERS,
        valueLetters = CURL_VALUE_LETTERS,
    )
val WGET =
    Client(
        valueFlags = WGET_VALUE_FLAGS,
        plainFlags = WGET_PLAIN_FLAGS,
        plainWords = WGET_PLAIN_WORDS,
        optionFiles = WGET_OPTION_FILES,
        plainLetters = WGET_PLAIN_LETTERS,
        valueLetters = WGET_VALUE_LETTERS,
    )

class ValueLetter(
    val letter: Char,
    val last: Boolean,
)

fun Client.valueLetter(flag: String): ValueLetter? {
    val bundle = flag.removePrefix("-").takeIf { flag.startsWith('-') && !it.startsWith('-') } ?: return null
    val offset = bundle.indexOfFirst { it in valueLetters }
    return if (offset < 0) null else ValueLetter(bundle[offset], offset + 1 == bundle.length)
}

fun Client.takesNextWord(flag: String) =
    if (flag.startsWith("--")) {
        flag in valueFlags || flag in METHOD_FLAGS || flag in BODY_FLAGS || flag in UPLOAD_FLAGS
    } else {
        valueLetter(flag)?.last == true
    }

fun Client.knows(flag: String) =
    when {
        flag == "-" || flag == "--" || flag in plainWords -> true
        flag.startsWith("--") -> knowsLong(flag.substringBefore('='))
        else -> knowsLetters(flag.drop(1))
    }

fun Client.knowsLong(stem: String): Boolean {
    val known = listOf(valueFlags, plainFlags, optionFiles, METHOD_FLAGS, BODY_FLAGS, UPLOAD_FLAGS)
    return known.any { stem in it }
}

fun Client.knowsLetters(letters: String): Boolean {
    val first = letters.indexOfFirst { it in valueLetters || it !in plainLetters }
    return first < 0 || letters[first] in valueLetters
}

fun Client.readsOptions(flag: String) =
    if (flag.startsWith("--")) {
        flag.substringBefore('=') in optionFiles
    } else {
        valueLetter(flag)?.let { "-${it.letter}" in optionFiles } == true
    }
