package dev.yuyuyuyuyu.claudeghadmissionhook

import io.matthewnelson.kmp.file.IOException
import io.matthewnelson.kmp.file.toFile
import io.matthewnelson.kmp.process.Process
import io.matthewnelson.kmp.process.Stdio
import io.matthewnelson.kmp.process.changeDir

private const val PATIENCE_SECONDS = 10
private const val MILLISECONDS_PER_SECOND = 1000
private const val SHOWN_LENGTH = 200

class Answer(
    val stdout: String,
    private val stderr: String,
    val status: Int,
) {
    fun failure(): String {
        val said = stderr.split('\n').map { it.removeSuffix("\r").trimWhiteSpace() }.find { it.isNotEmpty() }
        return if (said == null) "failed without saying why" else "failed (${said.take(SHOWN_LENGTH)})"
    }
}

class Unanswered(
    val failure: String,
    cause: Throwable? = null,
) : Exception(failure, cause)

fun run(
    command: String,
    arguments: List<String>,
    directory: String? = null,
    removed: List<String> = emptyList(),
): Answer {
    val output =
        try {
            Process
                .Builder(command)
                .args(arguments)
                .changeDir(directory?.toFile())
                .environment { removed.forEach { remove(it) } }
                .stdin(Stdio.Null)
                .createOutput { timeoutMillis = PATIENCE_SECONDS * MILLISECONDS_PER_SECOND }
        } catch (error: IOException) {
            throw Unanswered("could not be started (${error.message})", error)
        }
    if (output.processError != null) throw Unanswered("did not answer within $PATIENCE_SECONDS seconds")
    return Answer(output.stdout, output.stderr, output.processInfo.exitCode)
}

fun ask(arguments: List<String>) = run("gh", arguments, removed = listOf("GH_FORCE_TTY", "CLICOLOR_FORCE"))
