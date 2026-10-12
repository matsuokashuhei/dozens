import AVKit
import DesignSystem
import Domain
import SwiftUI

struct CypherPlayerView: View {
    @State private var model: CypherPlayerModel

    init(model: CypherPlayerModel) {
        _model = State(initialValue: model)
    }

    var body: some View {
        content
            .background(.black)
            .navigationTitle(Text(model.cypher?.beat.title ?? ""))
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            .toolbar(.hidden, for: .tabBar)
            #endif
            .task { await model.load() }
            .task(id: model.currentRapID) { await model.playCurrentRap() }
            .onDisappear { model.player.pause() }
            .toolbar {
                if let onHookUp = model.onHookUp, let cypher = model.cypher, let rap = model.currentRap {
                    ToolbarItem(placement: .primaryAction) {
                        Button {
                            model.player.pause()
                            onHookUp(cypher, rap)
                        } label: {
                            Label {
                                Text("Hook Up", bundle: .module)
                            } icon: {
                                Image(systemName: "plus.circle")
                            }
                        }
                        .accessibilityIdentifier("player.hookUp")
                    }
                }
            }
    }

    @ViewBuilder
    private var content: some View {
        if model.cypher != nil {
            ScrollView(.horizontal) {
                LazyHStack(spacing: 0) {
                    ForEach(Array(model.raps.enumerated()), id: \.element.id) { index, rap in
                        page(rap, index: index)
                            .containerRelativeFrame([.horizontal, .vertical])
                    }
                }
                .scrollTargetLayout()
            }
            .scrollTargetBehavior(.paging)
            .scrollPosition(id: $model.currentRapID)
            .scrollIndicators(.hidden)
        } else if model.error != nil {
            ContentUnavailableView {
                Label {
                    Text("Couldn't load the cypher.", bundle: .module)
                } icon: {
                    Image(systemName: "exclamationmark.triangle")
                }
            } actions: {
                Button {
                    Task { await model.load() }
                } label: {
                    Text("Retry", bundle: .module)
                }
            }
        } else {
            ProgressView()
                .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
    }

    private func page(_ rap: Rap, index: Int) -> some View {
        ZStack(alignment: .bottomLeading) {
            if rap.id == model.currentRapID {
                VideoPlayer(player: model.player)
                    .accessibilityIdentifier("player.video")
            } else {
                RapThumbnail(url: rap.thumbnailURL)
            }
            VStack(alignment: .leading, spacing: 4) {
                Text(verbatim: "@\(rap.rapper.name)")
                    .font(.dozens(.headingSmall))
                Text(verbatim: "\(index + 1) / \(model.raps.count)")
                    .font(.dozens(.bodySmall))
                    .monospacedDigit()
                    .accessibilityIdentifier("player.position")
            }
            .foregroundStyle(.white)
            .padding(12)
            .dozensGlassEffect(in: .rect(cornerRadius: 12))
            .padding()
            .allowsHitTesting(false)
        }
    }
}
