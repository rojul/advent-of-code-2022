import java.io.File

fun readInput(day: Int) = File("inputs", "${"$day".padStart(2, '0')}.txt")
    .readText()
    .replace("\r", "")
    .removeSuffix("\n")
