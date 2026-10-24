public struct OneTimeCode: Hashable, Sendable {
    public static let length = 6

    public let rawValue: String

    public init(_ raw: String) throws(DomainError) {
        guard raw.count == Self.length, raw.allSatisfy(\.isASCIIDigit) else { throw .invalidCode }
        rawValue = raw
    }
}

extension Character {
    var isASCIIDigit: Bool { isASCII && isNumber }
}
