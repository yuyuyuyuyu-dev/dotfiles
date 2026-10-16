package dev.yuyuyuyuyu.claudeghadmissionhook

private const val PULL_REQUEST_TOOLS = "mcp__ccd_pr__"

private val PULL_REQUEST_TOOLS_THAT_LEAVE_GITHUB_ALONE =
    listOf(
        "mcp__ccd_pr__bind_pr",
        "mcp__ccd_pr__get_status",
        "mcp__ccd_pr__set_monitor",
        "mcp__ccd_pr__unbind_pr",
    )

fun checkTool(name: String): Verdict? {
    if (!name.startsWith(PULL_REQUEST_TOOLS) || name in PULL_REQUEST_TOOLS_THAT_LEAVE_GITHUB_ALONE) return null
    return Verdict.Deny(
        "`$name` is not on this hook's allow-list of the pull request tools of the Claude desktop app that leave " +
            "GitHub unchanged, so it cannot be shown to leave GitHub unchanged. If `$name` really leaves GitHub " +
            "unchanged, the allow-list in claude-gh-admission-hook is what needs the entry.",
    )
}
