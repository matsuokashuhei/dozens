import SwiftUI

public struct DozensLogo: View {
    public init() {}

    public var body: some View {
        Image("Logo", bundle: .module)
            .resizable()
            .scaledToFit()
            .accessibilityLabel(Text(verbatim: "Dozens"))
    }
}

#Preview {
    DozensLogo()
        .frame(width: 160)
        .padding()
        .background(.dozensBackground)
}
