package dev.yuyuyuyuyu.claudeghadmissionhook

private val PUSH_TAG_FLAGS = listOf("--tags", "--follow-tags", "--mirror")
private val PUSH_VALUE_FLAGS =
    listOf(
        "-o",
        "--push-option",
        "--repo",
        "--receive-pack",
        "--exec",
        "--recurse-submodules",
    )
private const val PUSH_SETTINGS = """^(push\.followtags|remote\..*\.(mirror|push)|branch\..*\.merge)$"""
private val FALSE_VALUES = listOf("false", "no", "off", "")
private const val BRANCH_REF = "refs/heads/"

class Push(
    val globals: List<String>,
    val bare: Boolean,
)

fun checkPush(
    globals: List<Token>,
    rest: List<Token>,
): Verdict =
    checkPushedTags(rest)
        ?: checkPushedWords(rest)
        ?: refspecs(rest).find { !reachesABranch(it) }?.let { refspec ->
            Verdict.Deny(
                "`git push` names $refspec, and a bare name reaches a tag when a tag has that name, so it cannot be " +
                    "shown that no tag is sent to the remote. Push the branch that is checked out with " +
                    "`git push origin HEAD`, or name the branch in full, as in refs/heads/NAME or " +
                    "SOURCE:refs/heads/NAME.",
            )
        } ?: globals.find { !literal(it) }?.let { token ->
        Verdict.Deny(
            "${shown(token)} comes ahead of `git push` and out of a shell expansion, so the git configuration " +
                "the push would run under cannot be looked up. Write it out literally.",
        )
    } ?: Verdict.PassIfPushLeavesTags(Push(globals.map { it.value }, bare = refspecs(rest).isEmpty()))

private fun checkPushedTags(rest: List<Token>): Verdict? {
    val found = matched(flagNames(rest), PUSH_TAG_FLAGS)
    return when {
        found != null ->
            Verdict.Deny(
                "`git push` was given $found, which sends tags to the remote and so creates them there. Push the " +
                    "branch on its own, and ask the user to push tags.",
            )
        rest.any { TAG_REF in it.value } ->
            Verdict.Deny(
                "`git push` names a refs/tags/ refspec, which creates a tag on the remote even when no local tag " +
                    "exists. Push to refs/heads/ instead, and ask the user to push tags.",
            )
        else -> null
    }
}

private fun checkPushedWords(rest: List<Token>) =
    rest.find { splits(it) }?.let { splitting("push", it, "no tag is sent to the remote") }
        ?: rest.find { !literal(it) }?.let { token ->
            Verdict.Deny(
                "An argument of `git push` (${shown(token)}) comes out of a shell expansion, so it could turn out to " +
                    "be --tags or a refs/tags/ refspec, and it cannot be shown that no tag is sent to the remote. " +
                    "Write the remote and the branch out literally, or push HEAD.",
            )
        }

private fun refspecs(args: List<Token>): List<String> {
    val words = mutableListOf<String>()
    var flagsEnded = false
    val rest = args.iterator()

    while (rest.hasNext()) {
        val value = rest.next().value
        when {
            flagsEnded || !value.startsWith('-') -> words += value
            value == "--" -> flagsEnded = true
            value in PUSH_VALUE_FLAGS && rest.hasNext() -> rest.next()
        }
    }

    return words.drop(1)
}

private fun reachesABranch(given: String): Boolean {
    val refspec = given.removePrefix("+")
    return if (':' in refspec) {
        refspec.substringAfter(':').startsWith(BRANCH_REF)
    } else {
        refspec == "HEAD" || refspec.startsWith(BRANCH_REF)
    }
}

fun judge(
    push: Push,
    directory: String?,
): Verdict? {
    val found =
        try {
            settings(push.globals, PUSH_SETTINGS, directory)
        } catch (unanswered: Unanswered) {
            return Verdict.Deny(
                "`git push` is let through only after this hook has looked at the git configuration it would run " +
                    "under, and that could not be done: `git config` ${unanswered.failure}.",
            )
        }
    return found.split('\u0000').filter { it.isNotEmpty() }.firstNotNullOfOrNull { judgeSetting(it, push.bare) }
}

private fun judgeSetting(
    setting: String,
    bare: Boolean,
): Verdict? {
    val key = setting.substringBefore('\n')
    val value = if ('\n' in setting) setting.substringAfter('\n') else null
    return checkSentTags(key, value) ?: if (bare) checkPushDefault(key, value.orEmpty()) else null
}

private fun checkSentTags(
    key: String,
    value: String?,
) = when {
    key == "push.followtags" && on(value) ->
        Verdict.Deny(
            "`git push` would also send tags, because push.followTags is on in the git configuration, and sending " +
                "a tag creates it on the remote. Ask the user to push, or to turn push.followTags off.",
        )
    key.startsWith("remote.") && key.endsWith(".mirror") && on(value) ->
        Verdict.Deny(
            "`git push` could mirror every ref, tags included, because $key is on in the git configuration. Ask " +
                "the user to push.",
        )
    else -> null
}

private fun checkPushDefault(
    key: String,
    shown: String,
) = when {
    key.startsWith("remote.") && key.endsWith(".push") && !reachesABranch(shown) ->
        Verdict.Deny(
            "`git push` names nothing to push, and $key in the git configuration makes that $shown, which does not " +
                "surely reach a branch. Name what to push, as in `git push origin HEAD`.",
        )
    key.startsWith("branch.") && key.endsWith(".merge") && !shown.startsWith(BRANCH_REF) ->
        Verdict.Deny(
            "`git push` names nothing to push, and $key in the git configuration points at $shown, which is not a " +
                "branch. Name what to push, as in `git push origin HEAD`.",
        )
    else -> null
}

private fun on(given: String?): Boolean {
    val value = given?.lowercase() ?: return true
    return value !in FALSE_VALUES && value.toLongOrNull() != 0L
}
