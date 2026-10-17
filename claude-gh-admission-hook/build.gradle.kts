plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.detekt)
    alias(libs.plugins.ktlint)
}

detekt {
    source.setFrom("src")
}

kotlin {
    listOf(macosArm64(), macosX64(), linuxX64(), linuxArm64()).forEach { target ->
        target.binaries.executable {
            entryPoint = "dev.yuyuyuyuyu.claudeghadmissionhook.main"
        }
    }

    sourceSets {
        commonMain.dependencies {
            implementation(libs.kmp.process)
            implementation(libs.kotlinx.serialization.json)
        }
    }
}
