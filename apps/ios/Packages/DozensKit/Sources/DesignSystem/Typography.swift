import SwiftUI

/// 旧アプリ (ikzo-ios) の UIFont.App.Size に合わせた Futura ベースのスケール。Dynamic Type に追従する。
public enum DozensTextStyle: Sendable {
    case headingLarge
    case headingMedium
    case headingSmall
    case headingLabel
    case bodyRegular
    case bodySmall

    var size: CGFloat {
        switch self {
        case .headingLarge: 21
        case .headingMedium: 19
        case .headingSmall: 17
        case .headingLabel: 15
        case .bodyRegular: 13
        case .bodySmall: 11
        }
    }

    var relativeTo: Font.TextStyle {
        switch self {
        case .headingLarge: .title2
        case .headingMedium: .title3
        case .headingSmall: .headline
        case .headingLabel: .subheadline
        case .bodyRegular: .body
        case .bodySmall: .caption
        }
    }

    var fontName: String {
        switch self {
        case .headingLarge, .headingMedium, .headingSmall, .headingLabel: "Futura-Medium"
        case .bodyRegular, .bodySmall: "Futura-Medium"
        }
    }
}

public extension Font {
    static func dozens(_ style: DozensTextStyle) -> Font {
        .custom(style.fontName, size: style.size, relativeTo: style.relativeTo)
    }
}
