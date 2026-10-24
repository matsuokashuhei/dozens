import DesignSystem
import Domain
import SwiftUI

public struct TimelineView: View {
    @Bindable private var model: TimelineModel
    @State private var playingEntry: TimelineEntry?

    public init(model: TimelineModel) {
        self.model = model
    }

    public var body: some View {
        NavigationStack {
            content
                .background(.dozensBackground)
                .navigationTitle(Text("Timeline", bundle: .module))
                .toolbar {
                    ToolbarItem(placement: .principal) {
                        Picker(selection: $model.feed) {
                            ForEach(TimelineFeed.allCases, id: \.self) { feed in
                                feed.title.tag(feed)
                            }
                        } label: {
                            Text("Feed", bundle: .module)
                        }
                        .pickerStyle(.segmented)
                        .accessibilityIdentifier("timeline.feed")
                    }
                }
                .navigationDestination(item: $playingEntry) { entry in
                    CypherPlayerView(model: model.playerModel(for: entry))
                }
        }
        .task(id: model.feed) { await model.reload() }
    }

    @ViewBuilder
    private var content: some View {
        if model.entries.isEmpty {
            if model.error != nil {
                ContentUnavailableView {
                    Label {
                        Text("Couldn't load the timeline.", bundle: .module)
                    } icon: {
                        Image(systemName: "exclamationmark.triangle")
                    }
                } actions: {
                    Button {
                        Task { await model.reload() }
                    } label: {
                        Text("Retry", bundle: .module)
                    }
                }
            } else if model.isLoaded {
                ContentUnavailableView {
                    Label {
                        Text("No Raps", bundle: .module)
                    } icon: {
                        Image(systemName: "music.mic")
                    }
                }
            } else {
                ProgressView()
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            }
        } else {
            List(model.entries) { entry in
                Button {
                    playingEntry = entry
                } label: {
                    TimelineEntryRow(entry: entry)
                }
                .buttonStyle(.plain)
                .listRowBackground(Color.clear)
                .accessibilityIdentifier("timeline.entry.\(entry.id)")
                .task { await model.loadMore(ifReached: entry) }
            }
            .listStyle(.plain)
            .scrollContentBackground(.hidden)
            .refreshable { await model.reload() }
        }
    }
}

private extension TimelineFeed {
    var title: Text {
        switch self {
        case .following: Text("Following", bundle: .module)
        case .featured: Text("Featured", bundle: .module)
        case .newArrivals: Text("New Arrivals", bundle: .module)
        }
    }
}
