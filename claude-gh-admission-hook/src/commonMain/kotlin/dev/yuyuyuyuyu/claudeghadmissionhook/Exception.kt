package dev.yuyuyuyuyu.claudeghadmissionhook

private val REPO_FLAGS = listOf("-R", "--repo")
private val DRAFT_FLAGS = listOf("-d", "--draft")
private val TRUE_VALUES = listOf("1", "t", "T", "true", "TRUE", "True")

private val REPOSITORY = Regex("""^([A-Za-z0-9.-]+)/([A-Za-z0-9._-]+)/([A-Za-z0-9._-]+)\z""")
private val TAG = Regex("""^[A-Za-z0-9._/+@-]+\z""")
private val NUMBER = Regex("""^[1-9][0-9]{0,8}\z""")

class ExceptionalCommand(
    val path: List<String>,
    val valueFlags: List<String>,
    val boolFlags: List<String>,
    val judge: (Arguments) -> Verdict,
)

private val COMMANDS =
    listOf(
        ExceptionalCommand(
            path = listOf("pr", "create"),
            valueFlags =
                listOf(
                    "-a",
                    "--assignee",
                    "--attach",
                    "-B",
                    "--base",
                    "-b",
                    "--body",
                    "-F",
                    "--body-file",
                    "-H",
                    "--head",
                    "-l",
                    "--label",
                    "-m",
                    "--milestone",
                    "-p",
                    "--project",
                    "--recover",
                    "-r",
                    "--reviewer",
                    "-T",
                    "--template",
                    "-t",
                    "--title",
                    "-R",
                    "--repo",
                ),
            boolFlags =
                listOf(
                    "-d",
                    "--draft",
                    "--dry-run",
                    "-e",
                    "--editor",
                    "-f",
                    "--fill",
                    "--fill-first",
                    "--fill-verbose",
                    "--no-maintainer-edit",
                    "-w",
                    "--web",
                    "-h",
                    "--help",
                ),
            judge = ::createdPullRequest,
        ),
        ExceptionalCommand(
            path = listOf("pr", "edit"),
            valueFlags =
                listOf(
                    "--add-assignee",
                    "--add-label",
                    "--add-project",
                    "--add-reviewer",
                    "--attach",
                    "-B",
                    "--base",
                    "-b",
                    "--body",
                    "-F",
                    "--body-file",
                    "-m",
                    "--milestone",
                    "--remove-assignee",
                    "--remove-label",
                    "--remove-project",
                    "--remove-reviewer",
                    "-t",
                    "--title",
                    "-R",
                    "--repo",
                ),
            boolFlags = listOf("--remove-milestone", "-h", "--help"),
            judge = ::editedPullRequest,
        ),
        ExceptionalCommand(
            path = listOf("release", "create"),
            valueFlags =
                listOf(
                    "--discussion-category",
                    "-n",
                    "--notes",
                    "-F",
                    "--notes-file",
                    "--notes-start-tag",
                    "--target",
                    "-t",
                    "--title",
                    "-R",
                    "--repo",
                ),
            boolFlags =
                listOf(
                    "-d",
                    "--draft",
                    "--fail-on-no-commits",
                    "--generate-notes",
                    "--latest",
                    "--notes-from-tag",
                    "-p",
                    "--prerelease",
                    "--verify-tag",
                    "-h",
                    "--help",
                ),
            judge = ::createdRelease,
        ),
        ExceptionalCommand(
            path = listOf("release", "edit"),
            valueFlags =
                listOf(
                    "--discussion-category",
                    "-n",
                    "--notes",
                    "-F",
                    "--notes-file",
                    "--tag",
                    "--target",
                    "-t",
                    "--title",
                    "-R",
                    "--repo",
                ),
            boolFlags =
                listOf(
                    "--draft",
                    "--latest",
                    "--prerelease",
                    "--verify-tag",
                    "-h",
                    "--help",
                ),
            judge = ::editedRelease,
        ),
    )

class Value(
    val text: String,
    val literal: Boolean,
)

class Arguments {
    var draft: Boolean? = null
        private set
    private var repository: Value? = null
    val words = mutableListOf<String>()

    fun placed(
        command: ExceptionalCommand,
        name: String,
    ) {
        if (words.size >= command.path.size || name in REPO_FLAGS) return
        val named = command.path.joinToString(" ")
        throw Unreadable(
            "$name comes before `$named`, where gh may take the word after it for its value. Put it after `$named`.",
        )
    }

    fun record(
        name: String,
        value: Value?,
    ) {
        if (name in DRAFT_FLAGS) {
            draft = value == null || value.text in TRUE_VALUES
        } else if (name in REPO_FLAGS) {
            repository = value
        }
    }

    fun namedRepository(): Repository? {
        val located = repository?.takeIf { it.literal }?.let { REPOSITORY.find(it.text) } ?: return null
        val (host, owner, name) = located.destructured
        return Repository(host, owner, name)
    }
}

private class Unreadable(
    val problem: String,
) : Exception(problem)

fun exceptionalCommand(path: List<String>) =
    COMMANDS.find { command ->
        path.size >= command.path.size && path.zip(command.path).all { (word, expected) -> word == expected }
    }

fun checkExceptional(
    command: ExceptionalCommand,
    args: List<Token>,
): Verdict {
    val named = command.path.joinToString(" ")
    val arguments =
        try {
            ArgumentReader(command, args).read()
        } catch (unreadable: Unreadable) {
            return unreadable(named, unreadable.problem)
        }
    return if (arguments.words.take(command.path.size) == command.path) {
        command.judge(arguments)
    } else {
        unreadable(named, "`$named` could not be found among them.")
    }
}

private fun unreadable(
    named: String,
    problem: String,
) = Verdict.Deny(
    "`gh $named` is put to the user only when this hook can read every argument of it, and here it cannot: " +
        problem,
)

private class ArgumentReader(
    private val command: ExceptionalCommand,
    args: List<Token>,
) {
    private val rest = args.iterator()
    private val arguments = Arguments()
    private val named = command.path.joinToString(" ")
    private var flagsEnded = false

    fun read(): Arguments {
        while (rest.hasNext()) visit(rest.next())
        return arguments
    }

    private fun visit(token: Token) {
        val word = valueOf(token)
        val value = word.text
        when {
            flagsEnded || value == "-" || !value.startsWith('-') -> positional(token, word)
            value == "--" -> flagsEnded = true
            value.startsWith("--") -> longFlag(value, word)
            else -> shortFlags(value.substring(1), word)
        }
    }

    private fun positional(
        token: Token,
        word: Value,
    ) {
        if (!word.literal) {
            throw Unreadable(
                "${shown(token)} comes out of a shell expansion and is not the value of a flag, so it could turn " +
                    "out to be a flag. Write it out literally.",
            )
        }
        arguments.words += word.text
    }

    private fun longFlag(
        value: String,
        word: Value,
    ) {
        val name = value.substringBefore('=')
        val inline = if ('=' in value) Value(value.substringAfter('='), word.literal) else null
        arguments.placed(command, name)
        when (name) {
            in command.boolFlags -> arguments.record(name, inline)
            in command.valueFlags -> arguments.record(name, inline ?: following(name))
            else -> throw Unreadable(unknown(name))
        }
    }

    private fun shortFlags(
        letters: String,
        word: Value,
    ) {
        var offset = 0
        var done = false
        while (!done && offset < letters.length) {
            val width = widthAt(letters, offset)
            val name = "-" + letters.substring(offset, offset + width)
            offset += width
            done = shortFlag(name, letters.substring(offset), word)
        }
    }

    private fun shortFlag(
        name: String,
        after: String,
        word: Value,
    ): Boolean {
        arguments.placed(command, name)
        return when (name) {
            in command.boolFlags -> boolLetter(name, after, word)
            in command.valueFlags -> valueLetter(name, after, word)
            else -> throw Unreadable(unknown(name))
        }
    }

    private fun boolLetter(
        name: String,
        after: String,
        word: Value,
    ): Boolean {
        val inline = after.startsWith('=')
        arguments.record(name, if (inline) Value(after.substring(1), word.literal) else null)
        return inline
    }

    private fun valueLetter(
        name: String,
        after: String,
        word: Value,
    ): Boolean {
        arguments.record(name, if (after.isEmpty()) following(name) else Value(after.removePrefix("="), word.literal))
        return true
    }

    private fun following(name: String): Value {
        if (!rest.hasNext()) throw Unreadable("$name takes a value and none follows it.")
        return valueOf(rest.next())
    }

    private fun unknown(name: String) =
        "$name is not a flag this hook knows for `gh $named`, so it cannot tell whether the argument after it " +
            "is that flag's value. If $name is a real flag, the flag table in claude-gh-admission-hook is what " +
            "needs the entry."
}

private fun widthAt(
    letters: String,
    offset: Int,
): Int {
    val pair = letters[offset].isHighSurrogate() && letters.getOrNull(offset + 1)?.isLowSurrogate() == true
    return if (pair) 2 else 1
}

private fun valueOf(token: Token): Value {
    if (token.raw.startsWith('#')) {
        throw Unreadable(
            "a # follows the command, and what becomes of the text after it depends on the shell. Run the " +
                "command without the comment.",
        )
    }
    return when (shape(token.raw)) {
        Shape.LITERAL -> Value(token.value, literal = true)
        Shape.OPAQUE -> Value(token.value, literal = false)
        Shape.SPLITTING -> throw Unreadable(
            "${shown(token)} contains an unquoted shell expansion or pattern, which can turn into several " +
                "arguments. Quote it, or write the arguments out literally.",
        )
    }
}

private fun createdPullRequest(arguments: Arguments) =
    if (arguments.draft == true) {
        Verdict.Ask(
            "`gh pr create` would open a draft pull request. That is one of the few writes this hook allows, so " +
                "it needs the user to approve it rather than being denied.",
        )
    } else {
        Verdict.Deny(
            "`gh pr create` would open a pull request that is ready for review, because --draft is not in effect. " +
                "Only a draft pull request may be created: pass --draft, and leave marking it as ready to the user.",
        )
    }

private fun editedPullRequest(arguments: Arguments): Verdict {
    val number = arguments.words.getOrNull(2)?.takeIf { NUMBER.containsMatchIn(it) }
    val repository = arguments.namedRepository()
    return when {
        number == null ->
            Verdict.Deny(
                "`gh pr edit` is only allowed for a draft pull request, and this hook can only look one up by its " +
                    "number: with no number, or with a branch or a URL in its place, it cannot be sure which pull " +
                    "request the command would edit. Name the pull request by its number, as in " +
                    "`gh pr edit 123 -R github.com/OWNER/REPO`.",
            )
        repository == null ->
            Verdict.Deny(
                "`gh pr edit` is only allowed for a draft pull request, and this hook has to look the pull request " +
                    "up in the very repository the command would edit, which it can only be sure of when the " +
                    "command names it in full. Name that repository in full: pass -R HOST/OWNER/REPO, for example " +
                    "-R github.com/OWNER/REPO.",
            )
        else -> Verdict.AskIfDraftPullRequest(PullRequest(repository, number))
    }
}

private fun createdRelease(arguments: Arguments) =
    if (arguments.draft == true) {
        Verdict.Ask(
            "`gh release create` would save a draft release. That is one of the few writes this hook allows, so " +
                "it needs the user to approve it rather than being denied.",
        )
    } else {
        Verdict.Deny(
            "`gh release create` would publish a release, because --draft is not in effect. Only a draft release " +
                "may be created: pass --draft, and leave publishing it to the user.",
        )
    }

private fun editedRelease(arguments: Arguments): Verdict {
    val tag = arguments.words.getOrNull(2)
    val repository = arguments.namedRepository()
    return when {
        arguments.draft != null ->
            Verdict.Deny(
                "`gh release edit` was given --draft, which changes whether the release is published. Publishing " +
                    "a draft, or turning a published release back into a draft, is for the user to do. Leave " +
                    "--draft out: a draft release stays a draft when it is edited.",
            )
        tag == null ->
            Verdict.Deny(
                "`gh release edit` names no release, so there is nothing this hook can show to be a draft. Give " +
                    "the tag of the draft release to edit.",
            )
        !TAG.containsMatchIn(tag) ->
            Verdict.Deny(
                "`gh release edit` names the release $tag, and this hook only looks up tags made of letters, " +
                    "digits and the characters . _ / + @ - to find out whether a release is a draft. Ask the " +
                    "user to edit this one.",
            )
        repository == null ->
            Verdict.Deny(
                "`gh release edit` is only allowed for a draft release, and this hook has to look the release up " +
                    "in the very repository the command would edit, which it can only be sure of when the command " +
                    "names it in full. Name that repository in full: pass -R HOST/OWNER/REPO, for example " +
                    "-R github.com/OWNER/REPO.",
            )
        else -> Verdict.AskIfDraftRelease(Release(repository, tag))
    }
}
