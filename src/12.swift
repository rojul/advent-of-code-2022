import Utils

@main
enum Main {
    static func main() {
        let exampleMap = HeightMap(input: """
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
        """)
        assert(exampleMap.pathfind(start: [exampleMap.start]) == 31)
        assert(exampleMap.pathfind(start: exampleMap.coords("a")) == 29)

        let map = HeightMap(input: Utils.readInput(day: 12))
        print(map.pathfind(start: [map.start]))
        print(map.pathfind(start: map.coords("a")))
    }
}

struct HeightMap {
    let heights: [[UInt8]]
    let start: Coord
    let end: Coord

    init(input: String) {
        var start: Coord?
        var end: Coord?
        heights = input.components(separatedBy: .newlines).enumerated().map { y, line in
            line.enumerated().map { x, char in
                let height: Character
                switch char {
                case "a"..."z":
                    height = char
                case "S":
                    start = Coord(x: x, y: y)
                    height = "a"
                case "E":
                    end = Coord(x: x, y: y)
                    height = "z"
                default:
                    fatalError()
                }
                return height.asciiValue!
            }
        }
        self.start = start!
        self.end = end!
    }

    func pathfind(start: [Coord]) -> Int {
        var visited = Array(repeating: Array(repeating: false, count: heights[0].count), count: heights.count)
        var queue = start.map { ($0, steps: 0) }
        while true {
            let (coord, steps) = queue.removeFirst()
            for dir in Coord.dirs {
                let new = coord + dir
                guard heights.contains(new), !visited[new], heights[coord] + 1 >= heights[new] else {
                    continue
                }
                if new == end {
                    return steps + 1
                }
                visited[new.y][new.x] = true
                queue.append((new, steps + 1))
            }
        }
    }

    func coords(_ char: Character) -> [Coord] {
        let asciiValue = char.asciiValue
        return heights.enumerated().flatMap { y, row in
            row.enumerated().compactMap { x, height in
                height == asciiValue ? Coord(x: x, y: y) : nil
            }
        }
    }
}

struct Coord: Equatable {
    let x: Int
    let y: Int

    static func + (left: Coord, right: Coord) -> Coord {
        Coord(x: left.x + right.x, y: left.y + right.y)
    }

    static let dirs = [
        Coord(x: 0, y: -1),
        Coord(x: 1, y: 0),
        Coord(x: 0, y: 1),
        Coord(x: -1, y: 0),
    ]
}

extension Collection where Index == Int, Element: Collection, Element.Index == Int {
    func contains(_ coord: Coord) -> Bool {
        indices.contains(coord.y) && self[coord.y].indices.contains(coord.x)
    }

    subscript(_ coord: Coord) -> Element.Element {
        self[coord.y][coord.x]
    }
}
