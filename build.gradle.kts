plugins {
    id("com.bmuschko.docker-remote-api") version "6.7.0"
}

// Import task types
import com.bmuschko.gradle.docker.tasks.image.*

tasks.create("buildDocker", DockerBuildImage::class) {
    inputDir.set(file("$projectDir"))
    images.add("matrix-lightning-tip-bot")
}
