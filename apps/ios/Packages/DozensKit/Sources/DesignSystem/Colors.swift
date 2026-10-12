import SwiftUI

public extension Color {
    /// #2F2F2F
    static let dozensBlack = Color(red: 0x2F / 255, green: 0x2F / 255, blue: 0x2F / 255)
    /// #CB4D2B
    static let dozensRed = Color(red: 0xCB / 255, green: 0x4D / 255, blue: 0x2B / 255)
    /// #4EA87D
    static let dozensGreen = Color(red: 0x4E / 255, green: 0xA8 / 255, blue: 0x7D / 255)
    /// #9B9B9B
    static let dozensGray = Color(red: 0x9B / 255, green: 0x9B / 255, blue: 0x9B / 255)
    /// #E5E5E5
    static let dozensLightGray = Color(red: 0xE5 / 255, green: 0xE5 / 255, blue: 0xE5 / 255)
}

public extension ShapeStyle where Self == Color {
    static var dozensBackground: Color { .dozensBlack }
}
