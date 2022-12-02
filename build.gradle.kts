plugins {
    kotlin("jvm") version "1.7.22"
}

repositories {
    mavenCentral()
}

sourceSets {
    main {
        java.srcDirs("src")
    }
}

buildDir = File("target")
