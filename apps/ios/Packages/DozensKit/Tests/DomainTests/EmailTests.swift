import Domain
import Testing

struct EmailTests {
    @Test(arguments: ["rapper@example.com", "  rapper@example.com\n", "a.b+c@sub.example.co.jp"])
    func acceptsValidAddresses(raw: String) throws {
        let email = try Email(raw)
        #expect(email.rawValue == raw.trimmingCharacters(in: .whitespacesAndNewlines))
    }

    @Test(arguments: ["", "rapper", "rapper@", "@example.com", "rapper@example", "rap per@example.com", "a@b@c.com"])
    func rejectsInvalidAddresses(raw: String) {
        #expect(throws: DomainError.invalidEmail) { try Email(raw) }
    }
}

struct OneTimeCodeTests {
    @Test func acceptsSixDigits() throws {
        #expect(try OneTimeCode("012345").rawValue == "012345")
    }

    @Test(arguments: ["", "12345", "1234567", "12a456", "１２３４５６"])
    func rejectsOtherInput(raw: String) {
        #expect(throws: DomainError.invalidCode) { try OneTimeCode(raw) }
    }
}
