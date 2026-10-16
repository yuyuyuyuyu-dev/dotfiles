package dev.yuyuyuyuyu.claudeghadmissionhook

import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.addressOf
import kotlinx.cinterop.usePinned
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonObject
import platform.posix.EINTR
import platform.posix.STDIN_FILENO
import platform.posix.errno
import platform.posix.read

private const val READ_SIZE = 65536

fun main() {
    val payload = readStandardInput()?.let(::parse) ?: return
    val (decision, reason) = verdicts(payload)?.let { Decision(payload.field("cwd")?.text()).make(it) } ?: return
    print(
        buildJsonObject {
            putJsonObject("hookSpecificOutput") {
                put("hookEventName", "PreToolUse")
                put("permissionDecision", decision)
                put("permissionDecisionReason", reason)
            }
        },
    )
}

@OptIn(ExperimentalForeignApi::class)
private fun readStandardInput(): String? {
    val chunks = mutableListOf<ByteArray>()
    val buffer = ByteArray(READ_SIZE)
    var count: Long
    do {
        count = buffer.usePinned { read(STDIN_FILENO, it.addressOf(0), READ_SIZE.toULong()) }
        if (count > 0) chunks += buffer.copyOf(count.toInt())
    } while (count > 0 || (count < 0 && errno == EINTR))
    if (count < 0) return null
    return try {
        join(chunks).decodeToString(throwOnInvalidSequence = true)
    } catch (_: CharacterCodingException) {
        null
    }
}

private fun join(chunks: List<ByteArray>): ByteArray {
    val all = ByteArray(chunks.sumOf { it.size })
    var offset = 0
    for (chunk in chunks) {
        chunk.copyInto(all, offset)
        offset += chunk.size
    }
    return all
}

private fun parse(payload: String) =
    try {
        Json.parseToJsonElement(payload)
    } catch (_: IllegalArgumentException) {
        null
    }

private fun JsonElement.field(name: String) = (this as? JsonObject)?.get(name)

private fun JsonElement.text() = (this as? JsonPrimitive)?.takeIf { it.isString }?.content

private fun verdicts(payload: JsonElement): List<Verdict>? {
    val tool = checkTool(payload.field("tool_name")?.text().orEmpty())
    val command =
        payload
            .field("tool_input")
            ?.field("command")
            ?.text()
            .orEmpty()
    return when {
        tool != null -> listOf(tool)
        command.all { it.isUnicodeWhiteSpace() } -> null
        else -> checkCommand(command, 0)
    }
}
