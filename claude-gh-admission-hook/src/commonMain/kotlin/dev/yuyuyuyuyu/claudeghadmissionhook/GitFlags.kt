package dev.yuyuyuyuyu.claudeghadmissionhook

fun splitting(
    name: String,
    token: Token,
    harm: String,
) = Verdict.Deny(
    "An argument of `git $name` contains an unquoted shell expansion or pattern (${shown(token)}), which can turn " +
        "into further arguments, so it cannot be shown that $harm. Quote it, or write it out literally.",
)

fun flagNames(args: List<Token>): Set<String> {
    val names = mutableSetOf<String>()
    for (token in args.map { it.value }.filter { it.startsWith('-') }) {
        names += token.substringBefore('=')
        val count = token.removePrefix("-n")
        if (token.startsWith("-n") && count.isNotEmpty() && count.all { it in '0'..'9' }) names += "-n"
    }
    return names
}

fun matched(
    flags: Set<String>,
    against: List<String>,
): String? {
    val found =
        against
            .filter { known -> flags.any { flag -> flag == known || abbreviates(flag, known) } }
            .sorted()
    return if (found.isEmpty()) null else found.joinToString(" ")
}

private fun abbreviates(
    flag: String,
    known: String,
) = flag.length > 2 && flag.startsWith("--") && known.startsWith(flag)
