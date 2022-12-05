fun main() {
    val exampleStacksInput = """
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
    """.trimIndent()

    check(Stacks.parse(exampleStacksInput) == Stacks(listOf(
        mutableListOf('Z', 'N'),
        mutableListOf('M', 'C', 'D'),
        mutableListOf('P'),
    )))

    val exampleInstructions = listOf(
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ).map(Instruction::parse)

    check(Stacks.parse(exampleStacksInput).apply(exampleInstructions, Stacks::mover9000).topLayer() == "CMZ")
    check(Stacks.parse(exampleStacksInput).apply(exampleInstructions, Stacks::mover9001).topLayer() == "MCD")

    val (stacksInput, instructionsInput) = readInput(5).split("\n\n")
    val instructions = instructionsInput.split('\n').map(Instruction::parse)

    println(Stacks.parse(stacksInput).apply(instructions, Stacks::mover9000).topLayer())
    println(Stacks.parse(stacksInput).apply(instructions, Stacks::mover9001).topLayer())
}

private data class Stacks(val stacks: List<MutableList<Char>>) {
    companion object {
        fun parse(s: String): Stacks {
            val layers = s.split('\n').dropLast(1).reversed()
            val width = (layers[0].length + 1) / 4
            val stacks = Stacks((0 until width).map { mutableListOf() })
            for (layer in layers) {
                for (stack in 0 until width) {
                    layer.getOrNull(stack * 4 + 1)?.takeIf { it != ' ' }?.let {
                        stacks.stacks[stack] += it
                    }
                }
            }
            return stacks
        }
    }

    fun mover9000(instruction: Instruction) {
        repeat(instruction.quantity) {
            stacks[instruction.to] += stacks[instruction.from].removeLast()
        }
    }

    fun mover9001(instruction: Instruction) {
        val crates = stacks[instruction.from].takeLast(instruction.quantity)
        repeat(instruction.quantity) {
            stacks[instruction.from].removeLast()
        }
        stacks[instruction.to] += crates
    }

    fun apply(instructions: Iterable<Instruction>, mover: Stacks.(Instruction) -> Unit) = apply {
        for (instruction in instructions) {
            mover(instruction)
        }
    }

    fun topLayer() = String(stacks.map { it.last() }.toCharArray())
}

private data class Instruction(val quantity: Int, val from: Int, val to: Int) {
    companion object {
        fun parse(s: String): Instruction {
            val parts = s.split(' ')
            check(parts.size == 6)
            return Instruction(parts[1].toInt(), parts[3].toInt() - 1, parts[5].toInt() - 1)
        }
    }
}
