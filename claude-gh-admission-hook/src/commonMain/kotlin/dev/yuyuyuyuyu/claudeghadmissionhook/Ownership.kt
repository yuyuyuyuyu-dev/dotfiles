package dev.yuyuyuyuyu.claudeghadmissionhook

private const val OWNERS =
    "query(\$owner: String!, \$name: String!) { viewer { login } " +
        "repository(owner: \$owner, name: \$name) { owner { login } } }"
private const val VIEWER_AND_OWNER = """.data.viewer.login + " " + .data.repository.owner.login"""

fun judgeOwnership(
    repository: Repository,
    command: String,
): Verdict.Deny? {
    val (viewer, owner) =
        try {
            loginsOf(repository)
        } catch (unanswered: Unanswered) {
            return Verdict.Deny(
                "`$command` is only allowed in a repository the user owns, and who owns $repository could not be " +
                    "found out: `gh api graphql` ${unanswered.failure}. Check the repository, and that gh is " +
                    "signed in to ${repository.host}.",
            )
        }
    return if (viewer.equals(owner, ignoreCase = true)) {
        null
    } else {
        Verdict.Deny(
            "$repository is owned by $owner, not by $viewer, whom gh is signed in as on ${repository.host}. " +
                "`$command` is only allowed in a repository the user owns. A pull request in any other " +
                "repository, such as the one a fork was made from, is for the user to make.",
        )
    }
}

private fun loginsOf(repository: Repository): Pair<String, String> {
    val answer =
        ask(
            listOf(
                "api",
                "--hostname",
                repository.host,
                "graphql",
                "-f",
                "query=$OWNERS",
                "-f",
                "owner=${repository.owner}",
                "-f",
                "name=${repository.name}",
                "--jq",
                VIEWER_AND_OWNER,
            ),
        )
    val logins = answer.stdout.trimWhiteSpace().split(' ')
    if (answer.status != 0 || logins.size != 2 || logins.any { it.isEmpty() }) throw Unanswered(answer.failure())
    return logins[0] to logins[1]
}
