package dev.yuyuyuyuyu.claudeghadmissionhook

private val PUSH_FORCE_FLAGS = listOf("--force", "--force-with-lease")

fun forces(rest: List<Token>) =
    matched(flagNames(rest), PUSH_FORCE_FLAGS) != null ||
        rest.map { it.value }.takeWhile { it != "--" }.any { value ->
            !value.startsWith("--") && value.startsWith('-') && 'f' in value.drop(1).substringBefore('o')
        }

fun forcedPush(cause: String) =
    Verdict.Ask(
        "$cause, so it can replace commits that are already on the remote branch, as a push after " +
            "`git commit --amend` does. Pushing a topic branch is let through, but a forced push needs the user " +
            "to approve it.",
    )
