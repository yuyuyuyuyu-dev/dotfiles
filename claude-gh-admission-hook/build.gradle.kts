plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.detekt)
    alias(libs.plugins.ktlint)
    alias(libs.plugins.version.catalog.update)
}

detekt {
    source.setFrom("src")
}

ktlint {
    version = libs.versions.ktlintCli.get()
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
