package dev.yuyuyuyuyu.claudeghadmissionhook

class Repository(
    val host: String,
    val owner: String,
    val name: String,
) {
    override fun toString() = "$host/$owner/$name"
}
