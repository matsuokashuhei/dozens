import SwiftUI

/// Liquid Glass は visionOS で使えないので、visionOS ではそれぞれ同等の標準スタイルに置き換える。
public extension View {
    func dozensGlassEffect(in shape: some InsettableShape) -> some View {
        #if os(visionOS)
        glassBackgroundEffect(in: shape)
        #else
        glassEffect(.regular, in: shape)
        #endif
    }

    func dozensProminentButtonStyle() -> some View {
        #if os(visionOS)
        buttonStyle(.borderedProminent)
        #else
        buttonStyle(.glassProminent)
        #endif
    }
}
