package dev.yuyuyuyuyu.claudeghadmissionhook

private val PUSH_BRANCHES_FLAGS = listOf("--all", "--branches", "--repo")
private val BRANCH_SWITCHERS = listOf("switch", "checkout", "rebase", "bisect", "symbolic-ref")
private val BRANCH_RENAME_FLAGS = listOf("-m", "-M", "--move")
private val UPSTREAM_DEFAULTS = listOf("upstream", "tracking")
private const val SYMREF = "ref: "
private const val DEFAULT_REMOTE = "origin"
private const val INSTEAD =
    "Only a non-default branch may be pushed: create one with `git switch -c BRANCH`, and push it with " +
        "$PUSH_COMMAND."

private class Unreached(
    val reason: String,
) : Exception(reason)

private class Settings(
    pairs: List<Pair<String, String>>,
) {
    private val values = pairs.groupBy({ it.first }, { it.second })

    fun all(key: String) = values[key].orEmpty()

    fun last(key: String) = values[key]?.lastOrNull()
}

fun checkSwitch(
    name: String,
    rest: List<Token>,
) = when {
    name in BRANCH_SWITCHERS -> {
        Verdict.Unsettling("may switch the branch that is checked out with `git $name`")
    }

    name == "branch" && matched(flagNames(rest), BRANCH_RENAME_FLAGS) != null -> {
        Verdict.Unsettling("may rename the branch that is checked out with `git branch`")
    }

    else -> {
        null
    }
}

fun checkPushedBranches(rest: List<Token>) =
    matched(flagNames(rest), PUSH_BRANCHES_FLAGS)?.let { found ->
        Verdict.Deny(
            "`git push` was given $found, which either pushes every branch, the default branch among them, or " +
                "names the remote in a way this hook does not read. Name the remote and the branch, as in " +
                "$PUSH_COMMAND where BRANCH is a non-default branch.",
        )
    }

fun checkDestination(
    push: Push,
    pairs: List<Pair<String, String>>,
    directory: String?,
): Verdict? =
    try {
        val settings = Settings(pairs)
        val current = currentBranch(push, directory)
        val remote = push.remote ?: remoteOf(settings, current)
        val pushed = settings.all("remote.$remote.push")
        val reached =
            if (push.bare) {
                configured(settings, pushed, current)
            } else {
                push.refspecs.flatMap { destinations(it, current, pushed) }
            }
        val default = defaultBranch(push, remote, directory)
        reached.find { it == default }?.let { branch ->
            Verdict.Deny("`git push` would push to $branch, which is the default branch of $remote. $INSTEAD")
        }
    } catch (unreached: Unreached) {
        Verdict.Deny(unreached.reason)
    } catch (unanswered: Unanswered) {
        Verdict.Deny(
            "`git push` is let through only after this hook has made sure that it does not push to the default " +
                "branch, and that could not be done: `git` ${unanswered.failure}.",
        )
    }

private fun currentBranch(
    push: Push,
    directory: String?,
): String? {
    val answer = run("git", push.globals + listOf("symbolic-ref", "--quiet", "HEAD"), directory)
    return when (answer.status) {
        0 -> answer.stdout.trimWhiteSpace().removePrefix(BRANCH_REF)
        1 -> null
        else -> throw Unanswered(answer.failure())
    }
}

private fun remoteOf(
    settings: Settings,
    current: String?,
) = current?.let { settings.last("branch.$it.pushremote") }
    ?: settings.last("remote.pushdefault")
    ?: current?.let { settings.last("branch.$it.remote") }
    ?: DEFAULT_REMOTE

private fun configured(
    settings: Settings,
    pushed: List<String>,
    current: String?,
): List<String> {
    if (pushed.isNotEmpty()) return pushed.map { destination(it, current) }
    return when (val mode = settings.last("push.default")?.lowercase() ?: "simple") {
        "nothing" -> {
            emptyList()
        }

        "matching" -> {
            throw Unreached(
                "`git push` names nothing to push, and push.default is $mode in the git configuration, so it " +
                    "would push every branch that the remote also has, the default branch among them. $INSTEAD",
            )
        }

        in UPSTREAM_DEFAULTS -> {
            listOf(
                settings.last("branch.${checkedOut(current)}.merge")?.removePrefix(BRANCH_REF)
                    ?: checkedOut(current),
            )
        }

        else -> {
            listOf(checkedOut(current))
        }
    }
}

private fun destinations(
    refspec: String,
    current: String?,
    pushed: List<String>,
): List<String> {
    val named = destination(refspec, current)
    if (':' in refspec) return listOf(named)
    val source = BRANCH_REF + named
    return listOf(pushed.firstNotNullOfOrNull { mapped(source, it) } ?: named)
}

private fun mapped(
    source: String,
    given: String,
): String? {
    val refspec = given.removePrefix("+")
    val from = refspec.substringBefore(':')
    val to = refspec.substringAfter(':', from)
    val prefix = from.substringBefore('*')
    val suffix = from.substringAfter('*', "")
    val matches =
        if ('*' in from) {
            source.length >= prefix.length + suffix.length && source.startsWith(prefix) && source.endsWith(suffix)
        } else {
            source == from
        }
    return if (matches) {
        to.replace("*", source.substring(prefix.length, source.length - suffix.length)).removePrefix(BRANCH_REF)
    } else {
        null
    }
}

private fun destination(
    given: String,
    current: String?,
): String {
    val refspec = given.removePrefix("+")
    val target = if (':' in refspec) refspec.substringAfter(':') else refspec
    if ('*' in target) {
        throw Unreached(
            "`git push` would push to $target, which matches every branch, the default branch among them. $INSTEAD",
        )
    }
    return if (target == "HEAD") checkedOut(current) else target.removePrefix(BRANCH_REF)
}

private fun checkedOut(current: String?) =
    current ?: throw Unreached(
        "`git push` would push the branch that is checked out, but HEAD is detached, so which branch it would " +
            "push to cannot be determined. $INSTEAD",
    )

private fun defaultBranch(
    push: Push,
    remote: String,
    directory: String?,
): String {
    val answer = run("git", push.globals + listOf("ls-remote", "--symref", remote, "HEAD"), directory)
    if (answer.status != 0) throw Unanswered(answer.failure())
    return answer.stdout
        .split('\n')
        .find { it.startsWith(SYMREF) }
        ?.removePrefix(SYMREF)
        ?.substringBefore('\t')
        ?.removePrefix(BRANCH_REF)
        ?: throw Unreached(
            "`git push` would push to $remote, which does not say which branch is its default branch, so it " +
                "cannot be shown that the push does not reach the default branch. Ask the user to push.",
        )
}
