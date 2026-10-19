package dev.yuyuyuyuyu.claudeghadmissionhook

private const val ALIAS_DEPTH = 3
private const val DENY_NOTE =
    " GitHub access is read-only under this hook: every write is denied, and the only exceptions are " +
        "creating a draft pull request, editing a draft pull request, creating a draft release and editing " +
        "a draft release, which are put to the user for approval instead. As commands, they are " +
        "`gh pr create --draft --title TITLE`, `gh pr edit NUMBER -R HOST/OWNER/REPO --body BODY` where " +
        "NUMBER is an open draft pull request, `gh release create TAG --draft --title TITLE`, and " +
        "`gh release edit TAG -R HOST/OWNER/REPO --notes NOTES` where TAG is the tag the draft release was " +
        "created with. This is a permanent PreToolUse hook, not a transient failure: retrying the command, " +
        "or rewording or wrapping it to do the same thing, will not change the answer. Running the command " +
        "the way this reason says to, where it says so, is not such a rewording but what this hook asks " +
        "for. Ask the user to run it themselves if the write is genuinely needed. If the command only reads " +
        "and this denial looks like a fault in the hook, report that to the user instead of working around " +
        "it."

private class Denied(
    val reason: String,
    cause: Throwable? = null,
) : Exception(reason, cause)

class Decision(
    private val directory: String?,
) {
    private var unsettling: String? = null
    private val pushes = mutableListOf<Push>()
    private val approvals = mutableListOf<Verdict>()

    fun make(verdicts: List<Verdict>): Pair<String, String>? =
        try {
            follow(verdicts)
            judgePushes()
            ask()
        } catch (denied: Denied) {
            "deny" to denied.reason + DENY_NOTE
        }

    private fun follow(verdicts: List<Verdict>) {
        var aliases = file(verdicts)
        var rounds = 0
        while (aliases.isNotEmpty()) {
            unsettling?.let { throw Denied(stale(it)) }
            rounds += 1
            if (rounds > ALIAS_DEPTH) {
                throw Denied(
                    "A git alias in this command leads to more aliases than this hook follows, so what it " +
                        "would run cannot be determined.",
                )
            }
            aliases = file(aliases.flatMap { resolveOrDeny(it) }.flatMap { checkCommand(it, 0) })
        }
    }

    private fun resolveOrDeny(alias: Alias) =
        try {
            resolve(alias, directory)
        } catch (unresolved: Unresolved) {
            throw Denied(unresolved.reason, unresolved)
        }

    private fun file(verdicts: List<Verdict>): List<Alias> {
        val aliases = mutableListOf<Alias>()
        for (verdict in verdicts) {
            when (verdict) {
                is Verdict.Deny -> throw Denied(verdict.reason)
                is Verdict.Unsettling -> unsettling = unsettling ?: verdict.what
                is Verdict.PassIfPushLeavesTags -> pushes += verdict.push
                is Verdict.PassIfNotAlias -> aliases += verdict.alias
                else -> approvals += verdict
            }
        }
        return aliases
    }

    private fun judgePushes() {
        val what = unsettling
        if (what != null && pushes.isNotEmpty()) throw Denied(stale(what))
        val denial = pushes.firstNotNullOfOrNull { judge(it, directory) }
        if (denial is Verdict.Deny) throw Denied(denial.reason)
    }

    private fun ask(): Pair<String, String>? {
        val reasons = mutableListOf<String>()
        for (approval in approvals) {
            when (val settled = settle(approval)) {
                is Verdict.Deny -> throw Denied(settled.reason)
                is Verdict.Ask -> if (settled.reason !in reasons) reasons += settled.reason
                else -> Unit
            }
        }
        return if (reasons.isEmpty()) null else "ask" to reasons.joinToString(" ")
    }

    private fun settle(approval: Verdict) =
        when (approval) {
            is Verdict.AskIfDraftPullRequest -> judge(approval.pullRequest)
            is Verdict.AskIfDraftRelease -> judge(approval.release)
            else -> approval
        }

    private fun stale(what: String) =
        "`git push` and git commands that may be aliases are let through only after this hook has looked at " +
            "the git configuration they would run under, and this command also $what, so that configuration " +
            "could be another one by the time they run. Run the git command in a call of its own."
}
