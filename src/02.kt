fun main() {
    val rounds = readInput(2)
        .split("\n")
        .map {
            val shapes = it.mapNotNull { Shape.from(it) }
            shapes[0] to shapes[1]
        }
    println(rounds.sumOf { it.second.outcome(it.first) + it.second.ordinal + 1 })
    println(rounds.sumOf {
        val delta = when (it.second) {
            Shape.Rock /* lose */ -> -1
            Shape.Paper /* draw */ -> 0
            Shape.Scissors /* win */ -> 1
        }
        val shape = Shape.values()[(it.first.ordinal + delta + 3) % 3]
        shape.outcome(it.first) + shape.ordinal + 1
    })
}

private enum class Shape {
    Rock, Paper, Scissors;

    companion object {
        fun from(char: Char) = when (char) {
            'A', 'X' -> Rock
            'B', 'Y' -> Paper
            'C', 'Z' -> Scissors
            else -> null
        }
    }

    fun outcome(other: Shape) = when {
        (ordinal - 1 + 3) % 3 == other.ordinal -> 6
        this == other -> 3
        else -> 0
    }
}
