package dev.yuyuyuyuyu.claudeghadmissionhook

private val CONFIG_READ_FLAGS =
    listOf(
        "--get",
        "--get-all",
        "--get-regexp",
        "--get-urlmatch",
        "--list",
        "-l",
    )
private val CONFIG_WRITE_FLAGS =
    listOf(
        "--add",
        "--replace-all",
        "--unset",
        "--unset-all",
        "--rename-section",
        "--remove-section",
        "--edit",
        "-e",
    )
private val CONFIG_VALUE_FLAGS =
    listOf(
        "-f",
        "--file",
        "--blob",
        "--type",
        "--default",
        "--comment",
        "--value",
    )
private val CONFIG_READ_ACTIONS = listOf("get", "list")
private val CONFIG_WRITE_ACTIONS = listOf("set", "unset", "edit", "rename-section", "remove-section")
private val REMOTE_WRITE_ACTIONS =
    listOf(
        "add",
        "rename",
        "remove",
        "rm",
        "set-head",
        "set-branches",
        "set-url",
        "prune",
        "update",
    )

fun checkConfig(args: List<Token>): Verdict? {
    val unsettling = Verdict.Unsettling("changes the git configuration with `git config`")
    return if (readsConfig(args)) null else unsettling
}

fun checkRemote(args: List<Token>): Verdict? {
    val unsettling = Verdict.Unsettling("changes a remote with `git remote`")
    return if (writesRemote(args)) unsettling else null
}

private fun readsConfig(args: List<Token>): Boolean =
    when {
        args.any { !literal(it) } || matched(flagNames(args), CONFIG_WRITE_FLAGS) != null -> false
        args.any { it.value in CONFIG_READ_FLAGS } -> true
        else -> readsConfigWith(configWords(args))
    }

private fun configWords(args: List<Token>): List<String> {
    val words = mutableListOf<String>()
    val rest = args.iterator()
    while (rest.hasNext()) {
        val value = rest.next().value
        when {
            value in CONFIG_VALUE_FLAGS -> if (rest.hasNext()) rest.next()
            !value.startsWith('-') -> words += value
        }
    }
    return words
}

private fun readsConfigWith(words: List<String>) =
    when (words.size) {
        0 -> true
        1 -> words[0] !in CONFIG_WRITE_ACTIONS
        else -> words[0] in CONFIG_READ_ACTIONS
    }

private fun writesRemote(args: List<Token>) =
    args.any { !literal(it) } ||
        args.find { !it.value.startsWith('-') }?.let { it.value in REMOTE_WRITE_ACTIONS } == true

fun settings(
    globals: List<String>,
    pattern: String,
    directory: String?,
): String {
    val answer = run("git", globals + listOf("config", "-z", "--get-regexp", pattern), directory)
    if (answer.status != 0 && answer.status != 1) throw Unanswered(answer.failure())
    return answer.stdout
}
