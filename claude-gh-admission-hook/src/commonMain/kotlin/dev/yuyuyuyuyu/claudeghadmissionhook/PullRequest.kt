package dev.yuyuyuyuyu.claudeghadmissionhook

private const val STATE_AND_DRAFT = """"\(.state) \(.isDraft)""""

class NewPullRequest(
    val repository: Repository,
)

class PullRequest(
    val repository: Repository,
    val number: String,
)

fun judge(newPullRequest: NewPullRequest): Verdict {
    val repository = newPullRequest.repository
    return judgeOwnership(repository, "gh pr create") ?: Verdict.Ask(
        "`gh pr create` would open a draft pull request in $repository, a repository the user owns. That is one " +
            "of the few writes this hook allows, so it needs the user to approve it rather than being denied.",
    )
}

fun judge(pullRequest: PullRequest): Verdict {
    val repository = pullRequest.repository
    val number = pullRequest.number
    val state =
        try {
            stateOf(pullRequest)
        } catch (unanswered: Unanswered) {
            return Verdict.Deny(
                "`gh pr edit` is only allowed for a draft pull request, and whether $number in $repository is " +
                    "one could not be found out: `gh pr view` ${unanswered.failure}. Check the number and the " +
                    "repository, and that gh is signed in to ${repository.host}.",
            )
        }
    return if (state == null) {
        judgeOwnership(repository, "gh pr edit") ?: Verdict.Ask(
            "Pull request $number in $repository is an open draft, in a repository the user owns. Editing a " +
                "draft pull request is one of the few writes this hook allows, so it needs the user to approve " +
                "it rather than being denied.",
        )
    } else {
        Verdict.Deny(
            "Pull request $number in $repository is $state, and only a draft pull request that is still open " +
                "may be edited. Changing this one is for the user to do.",
        )
    }
}

private fun stateOf(pullRequest: PullRequest): String? {
    val answer =
        ask(
            listOf(
                "pr",
                "view",
                "--repo",
                pullRequest.repository.toString(),
                "--json",
                "isDraft,state",
                "--jq",
                STATE_AND_DRAFT,
                "--",
                pullRequest.number,
            ),
        )
    return when (answer.stdout.trimWhiteSpace()) {
        "OPEN true" -> null
        "OPEN false" -> "ready for review"
        "CLOSED true", "CLOSED false" -> "closed"
        "MERGED true", "MERGED false" -> "merged"
        else -> throw Unanswered(answer.failure())
    }
}
