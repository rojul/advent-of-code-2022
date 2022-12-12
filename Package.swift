// swift-tools-version: 5.7

import PackageDescription

let package = Package(
    name: "advent-of-code-2022",
    products: [
        .library(
            name: "Utils",
            targets: ["Utils"]
        ),
    ],
    dependencies: [
    ],
    targets: [
        .target(
            name: "Utils",
            path: "src",
            sources: ["Utils.swift"]
        ),
        .day(12),
    ]
)

extension Target {
    static func day(_ day: Int) -> Target {
        let day = String(format: "%02d", day)
        return .executableTarget(
            name: day,
            dependencies: ["Utils"],
            path: "src",
            sources: ["\(day).swift"]
        )
    }
}
