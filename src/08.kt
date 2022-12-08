fun main() {
    check(Trees(listOf(
        listOf(0, 0, 2, 0, 0),
        listOf(0, 0, 1, 0, 0),
        listOf(8, 7, 0, 3, 4),
        listOf(0, 0, 5, 0, 0),
        listOf(0, 0, 6, 0, 0),
    )).sight(2, 2) == listOf(
        listOf(1, 2),
        listOf(3, 4),
        listOf(5, 6),
        listOf(7, 8),
    ))

    val trees = Trees(readInput(8))

    val part1 = trees.map { height, x, y ->
        trees.sight(x, y).any { it.all { it < height } }
    }.count { it }
    println(part1)

    val part2 = trees.map { height, x, y ->
        trees.sight(x, y)
            .map { it.indexOfFirst { it >= height }.takeIf { it != -1 }?.plus(1) ?: it.size }
            .fold(1, Int::times)
    }.max()
    println(part2)
}

private class Trees(val trees: List<List<Int>>) {
    constructor(s: String) : this(s.lines().map { it.map(Char::digitToInt) })

    fun <R> map(transform: (Int, Int, Int) -> R): List<R> {
        return trees.withIndex().flatMap { (y, row) ->
            row.withIndex().map { (x, height) ->
                transform(height, x, y)
            }
        }
    }

    fun sight(x: Int, y: Int) = listOf(
        (y - 1 downTo 0).map { trees[it][x] },
        (x + 1 until trees.size).map { trees[y][it] },
        (y + 1 until trees.size).map { trees[it][x] },
        (x - 1 downTo 0).map { trees[y][it] },
    )
}
