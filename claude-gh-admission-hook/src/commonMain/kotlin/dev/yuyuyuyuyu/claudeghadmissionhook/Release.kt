package dev.yuyuyuyuyu.claudeghadmissionhook

class Release(
    val repository: Repository,
    val tag: String,
)

fun judge(release: Release): Verdict {
    val repository = release.repository
    val tag = release.tag
    val published =
        try {
            if (isDraft(release)) {
                null
            } else {
                Verdict.Deny(
                    "Release $tag in $repository is published, and only a draft release may be edited. " +
                        "Changing a published release is for the user to do.",
                )
            }
        } catch (unanswered: Unanswered) {
            Verdict.Deny(
                "`gh release edit` is only allowed for a draft release, and whether $tag in $repository is one " +
                    "could not be found out: `gh release view` ${unanswered.failure}. Check the tag and the " +
                    "repository, and that gh is signed in to ${repository.host}.",
            )
        }
    return published ?: judgeTwin(release)
}

private fun judgeTwin(release: Release): Verdict {
    val repository = release.repository
    val tag = release.tag
    return try {
        if (hasPublishedTwin(release)) {
            Verdict.Deny(
                "$repository holds a draft release with the tag $tag and a published release with the same " +
                    "tag, and `gh release edit` could pick either of them. As long as both exist, editing the " +
                    "draft is for the user to do.",
            )
        } else {
            Verdict.Ask(
                "Release $tag in $repository is a draft, and no published release shares its tag. Editing a " +
                    "draft release is one of the few writes this hook allows, so it needs the user to approve " +
                    "it rather than being denied.",
            )
        }
    } catch (unanswered: Unanswered) {
        Verdict.Deny(
            "Release $tag in $repository is a draft, but whether a published release shares its tag could not " +
                "be found out: `gh api` ${unanswered.failure}. If one does, `gh release edit` could pick it " +
                "instead, so the edit is not let through.",
        )
    }
}

private fun isDraft(release: Release): Boolean {
    val answer =
        ask(
            listOf(
                "release",
                "view",
                "--repo",
                release.repository.toString(),
                "--json",
                "isDraft",
                "--jq",
                ".isDraft",
                "--",
                release.tag,
            ),
        )
    return when (answer.stdout.trimWhiteSpace()) {
        "true" -> true
        "false" -> false
        else -> throw Unanswered(answer.failure())
    }
}

private fun hasPublishedTwin(release: Release): Boolean {
    val repository = release.repository
    val endpoint = "repos/${repository.owner}/${repository.name}/releases/tags/${release.tag.replace("/", "%2F")}"
    val answer = ask(listOf("api", "--hostname", repository.host, "--include", "--jq", ".draft", endpoint))
    val lines =
        answer.stdout
            .split('\n')
            .map { it.trimWhiteSpace() }
            .filter { it.isNotEmpty() }
    val status =
        lines
            .firstOrNull()
            ?.split(Regex("""[$WHITE_SPACE]+"""))
            ?.getOrNull(1)
    return when (if (status == "200") lines.last() else status) {
        "404", "true" -> false
        "false" -> true
        else -> throw Unanswered(answer.failure())
    }
}
