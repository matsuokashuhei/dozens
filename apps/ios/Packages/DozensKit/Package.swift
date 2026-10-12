// swift-tools-version: 6.2

import PackageDescription

let uiSettings: [SwiftSetting] = [
    .defaultIsolation(MainActor.self),
    .enableUpcomingFeature("NonisolatedNonsendingByDefault"),
    .enableUpcomingFeature("InferIsolatedConformances"),
]

let coreSettings: [SwiftSetting] = [
    .enableUpcomingFeature("NonisolatedNonsendingByDefault"),
    .enableUpcomingFeature("InferIsolatedConformances"),
]

let package = Package(
    name: "DozensKit",
    defaultLocalization: "en",
    platforms: [.iOS(.v26), .macOS(.v26), .visionOS(.v26)],
    products: [
        .library(name: "AppFeature", targets: ["AppFeature"]),
    ],
    targets: [
        .target(name: "Domain", swiftSettings: coreSettings),
        .target(name: "Application", dependencies: ["Domain"], swiftSettings: coreSettings),
        .target(
            name: "Infrastructure",
            dependencies: ["Domain"],
            resources: [.copy("Resources/stub-rap.mp4")],
            swiftSettings: coreSettings
        ),
        .target(name: "DesignSystem", resources: [.process("Resources")], swiftSettings: uiSettings),
        .target(
            name: "AuthFeature",
            dependencies: ["Domain", "Application", "DesignSystem"],
            resources: [.process("Resources")],
            swiftSettings: uiSettings
        ),
        .target(
            name: "TimelineFeature",
            dependencies: ["Domain", "Application", "DesignSystem"],
            resources: [.process("Resources")],
            swiftSettings: uiSettings
        ),
        .target(
            name: "AppFeature",
            dependencies: ["Domain", "Application", "Infrastructure", "DesignSystem", "AuthFeature", "TimelineFeature"],
            resources: [.process("Resources")],
            swiftSettings: uiSettings
        ),
        .testTarget(name: "DomainTests", dependencies: ["Domain"], swiftSettings: coreSettings),
        .testTarget(
            name: "ApplicationTests",
            dependencies: ["Application", "Domain"],
            swiftSettings: coreSettings
        ),
        .testTarget(
            name: "InfrastructureTests",
            dependencies: ["Infrastructure", "Domain"],
            swiftSettings: coreSettings
        ),
        .testTarget(
            name: "AuthFeatureTests",
            dependencies: ["AuthFeature", "Application", "Domain"],
            swiftSettings: uiSettings
        ),
    ],
    swiftLanguageModes: [.v6]
)
