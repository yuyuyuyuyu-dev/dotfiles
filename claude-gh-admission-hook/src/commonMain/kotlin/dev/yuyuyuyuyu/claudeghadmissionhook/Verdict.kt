package dev.yuyuyuyuyu.claudeghadmissionhook

sealed interface Verdict {
    class Deny(
        val reason: String,
    ) : Verdict

    class Ask(
        val reason: String,
    ) : Verdict

    class AskIfDraftPullRequest(
        val pullRequest: PullRequest,
    ) : Verdict

    class AskIfDraftRelease(
        val release: Release,
    ) : Verdict

    class PassIfPushLeavesTags(
        val push: Push,
    ) : Verdict

    class PassIfNotAlias(
        val alias: Alias,
    ) : Verdict

    class Unsettling(
        val what: String,
    ) : Verdict
}
