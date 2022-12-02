fun main() {
    val elfs = readInput(1)
        .split("\n\n")
        .map { it.split("\n").map { it.toInt() } }
    println(elfs.maxOf { it.sum() })
    println(elfs.map { it.sum() }.sortedDescending().take(3).sum())
}
