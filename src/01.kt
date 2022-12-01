fun main() {
    val elfs = object {}.javaClass.getResource("/01.txt")!!.readText().trim('\n')
        .split("\n\n")
        .map { it.split("\n").map { it.toInt() } }
    println(elfs.maxOf { it.sum() })
    println(elfs.map { it.sum() }.sortedDescending().take(3).sum())
}
