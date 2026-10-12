import DesignSystem
import Domain
import SwiftUI

struct TimelineEntryRow: View {
    let entry: TimelineEntry

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Label(entry.message, systemImage: entry.kind == .cypher ? "plus.bubble" : "link")
                .font(.dozens(.bodySmall))
                .foregroundStyle(Color.dozensGray)
            HStack(spacing: 12) {
                Image(systemName: "person.crop.circle.fill")
                    .font(.system(size: 36))
                    .foregroundStyle(Color.dozensGray)
                VStack(alignment: .leading, spacing: 2) {
                    Text(verbatim: "@\(entry.rap.rapper.name)")
                        .font(.dozens(.headingLabel))
                    if let nickname = entry.rap.rapper.nickname {
                        Text(nickname)
                            .font(.dozens(.bodySmall))
                            .foregroundStyle(Color.dozensGray)
                    }
                }
                Spacer()
                Text(entry.rap.createdAt, format: .relative(presentation: .named))
                    .font(.dozens(.bodySmall))
                    .foregroundStyle(Color.dozensGray)
            }
            RapThumbnail(url: entry.rap.thumbnailURL)
                .aspectRatio(1, contentMode: .fit)
                .overlay {
                    Image(systemName: "play.fill")
                        .font(.system(size: 32))
                        .foregroundStyle(.white)
                        .padding(20)
                        .dozensGlassEffect(in: .circle)
                }
                .clipShape(.rect(cornerRadius: 16))
            HStack(spacing: 16) {
                statistic(entry.rap.statistics.views, systemImage: "eye", label: Text("Views", bundle: .module))
                statistic(entry.rap.statistics.hookups, systemImage: "link", label: Text("Hookups", bundle: .module))
                statistic(entry.rap.statistics.likes, systemImage: "heart", label: Text("Likes", bundle: .module))
                statistic(entry.rap.statistics.comments, systemImage: "bubble.right", label: Text("Comments", bundle: .module))
                Spacer()
                Label(entry.beat.title, systemImage: "music.note")
                    .lineLimit(1)
            }
            .font(.dozens(.bodySmall))
            .foregroundStyle(Color.dozensLightGray)
        }
        .padding(.vertical, 8)
        .contentShape(.rect)
    }

    private func statistic(_ value: Int, systemImage: String, label: Text) -> some View {
        Label {
            Text(value, format: .number.notation(.compactName))
        } icon: {
            Image(systemName: systemImage)
        }
        .accessibilityElement(children: .ignore)
        .accessibilityLabel(label)
        .accessibilityValue(Text(value, format: .number))
    }
}
