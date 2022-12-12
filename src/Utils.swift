import Foundation

public func readInput(day: Int) -> String {
    let day = String(format: "%02d", day)
    let path = ProcessInfo.processInfo.environment["AOC_PATH"] ?? "."
    var input = try! String(contentsOfFile: "\(path)/inputs/\(day).txt")
    if input.hasSuffix("\n") {
        input.removeLast()
    }
    return input
}
