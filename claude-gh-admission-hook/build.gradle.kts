import org.jetbrains.kotlin.konan.target.HostManager
import org.jetbrains.kotlin.konan.target.KonanTarget

plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.detekt)
    alias(libs.plugins.ktlint)
}

detekt {
    source.setFrom("src")
}

kotlin {
    val host =
        when (HostManager.host) {
            KonanTarget.MACOS_ARM64 -> macosArm64("host")
            KonanTarget.MACOS_X64 -> macosX64("host")
            KonanTarget.LINUX_X64 -> linuxX64("host")
            else -> error("Kotlin/Native cannot build claude-gh-admission-hook on this host")
        }
    host.binaries.executable {
        entryPoint = "dev.yuyuyuyuyu.claudeghadmissionhook.main"
    }

    sourceSets {
        commonMain.dependencies {
            implementation(libs.kmp.process)
            implementation(libs.kotlinx.serialization.json)
        }
    }
}
