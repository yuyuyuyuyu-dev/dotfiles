package dev.yuyuyuyuyu.claudeghadmissionhook

private val GLOBAL_VALUE_FLAGS =
    listOf(
        "-C",
        "-c",
        "--git-dir",
        "--work-tree",
        "--namespace",
        "--exec-path",
        "--super-prefix",
        "--config-env",
        "--attr-source",
    )

private val TAG_WRITE_FLAGS =
    listOf(
        "-a",
        "--annotate",
        "-s",
        "--sign",
        "-u",
        "--local-user",
        "-m",
        "--message",
        "-F",
        "--file",
        "-f",
        "--force",
        "-d",
        "--delete",
        "-e",
        "--edit",
        "--cleanup",
    )
private val TAG_LIST_FLAGS =
    listOf(
        "-l",
        "--list",
        "-n",
        "--contains",
        "--no-contains",
        "--points-at",
        "--merged",
        "--no-merged",
        "-v",
        "--verify",
    )
private val INPUT_FLAGS = listOf("--stdin")
private val PUSH_PLUMBING = listOf("send-pack", "http-push")
private val REF_WRITERS = listOf("update-ref", "symbolic-ref")
private val FETCHERS = listOf("fetch", "pull")
const val TAG_REF = "refs/tags/"

class Invocation(
    val globals: List<Token>,
    val name: String,
    val rest: List<Token>,
)

private sealed interface Found {
    class Command(
        val invocation: Invocation,
    ) : Found

    object Nothing : Found

    class Undeterminable(
        val token: Token,
    ) : Found
}

private fun subcommand(args: List<Token>): Found {
    var index = 0
    var found: Found? = null
    while (found == null && index < args.size) {
        val token = args[index]
        val value = token.value
        found =
            when {
                !literal(token) -> Found.Undeterminable(token)
                value in GLOBAL_VALUE_FLAGS -> args.getOrNull(index + 1)?.takeIf(::splits)?.let(Found::Undeterminable)
                value.startsWith('-') -> null
                else -> Found.Command(Invocation(args.subList(0, index), value, args.subList(index + 1, args.size)))
            }
        index += if (value in GLOBAL_VALUE_FLAGS) 2 else 1
    }
    return found ?: Found.Nothing
}

fun checkGit(
    args: List<Token>,
    aliases: Boolean,
): Verdict? =
    when (val found = subcommand(args)) {
        is Found.Command -> {
            checkInvocation(found.invocation, aliases)
        }

        Found.Nothing -> {
            null
        }

        is Found.Undeterminable -> {
            Verdict.Deny(
                "${shown(found.token)} comes before the end of the git subcommand and contains a shell expansion " +
                    "or pattern, so which subcommand git would run cannot be determined. Quote it if it is the " +
                    "value of a flag, and write the subcommand out literally.",
            )
        }
    }

private fun checkInvocation(
    invocation: Invocation,
    aliases: Boolean,
): Verdict? {
    val name = invocation.name
    val rest = invocation.rest
    return when (name) {
        "tag" -> {
            checkTag(rest)
        }

        "push" -> {
            checkPush(invocation.globals, rest)
        }

        "config" -> {
            checkConfig(rest)
        }

        "remote" -> {
            checkRemote(rest)
        }

        in REF_WRITERS -> {
            readsStandardInput(name, rest) ?: checkRefWriter(name, rest)
        }

        in FETCHERS -> {
            readsStandardInput(name, rest) ?: checkFetch(name, rest)
        }

        in PUSH_PLUMBING -> {
            Verdict.Deny(
                "`git $name` sends refs to a remote the way `git push` does, and this hook only reads `git push`, " +
                    "so it cannot be shown that no tag is sent. Use `git push`.",
            )
        }

        else -> {
            if (aliases) checkAlias(invocation) else null
        }
    }
}

private fun checkTag(rest: List<Token>): Verdict? {
    val flags = flagNames(rest)
    return matched(flags, TAG_WRITE_FLAGS)?.let { found ->
        Verdict.Deny(
            "`git tag` was given $found, which creates, deletes or rewrites a tag. Listing tags is allowed -- " +
                "`git tag -l`, `git tag --points-at`, `git describe --tags` all pass -- but making one is for the " +
                "user to do.",
        )
    } ?: rest.find { splits(it) }?.let { splitting("tag", it, "no tag is created or deleted") }
        ?: rest.find { mayBeFlag(it) }?.let { token ->
            Verdict.Deny(
                "An argument of `git tag` (${shown(token)}) comes out of a shell expansion and could turn out to be " +
                    "a flag that creates or deletes a tag. Give it as --flag=value, start it with literal text, or " +
                    "write it out literally.",
            )
        } ?: checkTagCreation(flags, rest)
}

private fun checkTagCreation(
    flags: Set<String>,
    rest: List<Token>,
) = if (flags.none { it in TAG_LIST_FLAGS } && rest.any { !it.value.startsWith('-') }) {
    Verdict.Deny(
        "`git tag` names a tag to create. Listing tags is allowed -- `git tag` with no argument, `git tag -l`, " +
            "`git describe --tags` -- but creating one is for the user to do.",
    )
} else {
    null
}

private fun readsStandardInput(
    name: String,
    rest: List<Token>,
) = if (matched(flagNames(rest), INPUT_FLAGS) != null) {
    Verdict.Deny(
        "`git $name` was given --stdin, which makes it read the refs to write from standard input, where this " +
            "hook cannot see them, so it cannot be shown that no tag is created. Name the refs on the command line.",
    )
} else {
    null
}

private fun checkRefWriter(
    name: String,
    rest: List<Token>,
): Verdict? =
    if (rest.any { TAG_REF in it.value }) {
        Verdict.Deny(
            "`git $name` writes a ref under refs/tags/, which creates a tag without `git tag` being involved. Ask " +
                "the user to create the tag.",
        )
    } else {
        rest.find { splits(it) }?.let { splitting(name, it, "no ref under refs/tags/ is written") }
            ?: checkExpandedRef(name, rest)
    }

private fun checkExpandedRef(
    name: String,
    rest: List<Token>,
) = rest.find { !literal(it) }?.let { token ->
    Verdict.Deny(
        "An argument of `git $name` (${shown(token)}) comes out of a shell expansion, so it could turn out to " +
            "name a ref under refs/tags/, and it cannot be shown that no tag is created. Write the ref out " +
            "literally.",
    )
}

private fun checkFetch(
    name: String,
    rest: List<Token>,
) = rest.find { makesATag(it.value) }?.let { token ->
    Verdict.Deny(
        "`git $name` was given ${shown(token)}, which writes a ref under refs/tags/ from another ref and so creates " +
            "a tag. Ask the user to create the tag.",
    )
}

private fun makesATag(given: String): Boolean {
    val refspec = given.removePrefix("+")
    val source = refspec.substringBefore(':')
    val destination = refspec.substringAfter(':', "")
    return ':' in refspec && destination.startsWith(TAG_REF) && source != destination
}
