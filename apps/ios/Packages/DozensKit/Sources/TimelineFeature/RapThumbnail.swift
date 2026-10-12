import DesignSystem
import SwiftUI

struct RapThumbnail: View {
    let url: URL?

    var body: some View {
        AsyncImage(url: url) { image in
            image.resizable().scaledToFill()
        } placeholder: {
            LinearGradient(colors: [.dozensRed, .dozensBlack], startPoint: .topLeading, endPoint: .bottomTrailing)
                .overlay {
                    Image(systemName: "waveform")
                        .font(.system(size: 48))
                        .foregroundStyle(.white.opacity(0.6))
                }
        }
        .clipped()
    }
}
