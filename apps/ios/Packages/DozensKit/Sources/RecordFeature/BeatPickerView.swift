import AVFoundation
import DesignSystem
import Domain
import SwiftUI

struct BeatPickerView: View {
    @Bindable var model: RecordModel
    @State private var preview = AVPlayer()
    @State private var previewingPartID: BeatPart.ID?

    var body: some View {
        content
            .navigationTitle(Text("Choose a Beat", bundle: .module))
            .task(id: model.hookUp) {
                if !model.isLoaded { await model.load() }
            }
            .onDisappear(perform: stopPreview)
    }

    @ViewBuilder
    private var content: some View {
        if !model.beats.isEmpty {
            list
        } else if model.error != nil {
            ContentUnavailableView {
                Label {
                    Text("Couldn't load the beats.", bundle: .module)
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
        } else if model.isLoaded {
            ContentUnavailableView {
                Label {
                    Text("No Beats", bundle: .module)
                } icon: {
                    Image(systemName: "music.note")
                }
            }
        } else {
            ProgressView()
                .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
    }

    private var list: some View {
        List {
            if model.hookUp != nil {
                Section {
                    HStack {
                        Label {
                            Text("Hooking up to a cypher", bundle: .module)
                        } icon: {
                            Image(systemName: "link")
                        }
                        Spacer()
                        Button {
                            model.start(hookUp: nil)
                        } label: {
                            Text("Cancel", bundle: .module)
                        }
                        .accessibilityIdentifier("beats.cancelHookUp")
                    }
                }
            }
            ForEach(model.beats) { beat in
                Section {
                    ForEach(beat.parts) { part in
                        row(part)
                    }
                } header: {
                    VStack(alignment: .leading) {
                        Text(verbatim: beat.title)
                            .font(.dozens(.headingSmall))
                        if let artist = beat.artistName {
                            Text(verbatim: artist)
                                .font(.dozens(.bodySmall))
                        }
                    }
                }
            }
        }
        .scrollContentBackground(.hidden)
    }

    private func row(_ part: BeatPart) -> some View {
        HStack(spacing: 12) {
            Button {
                togglePreview(part)
            } label: {
                Image(systemName: previewingPartID == part.id ? "stop.circle.fill" : "play.circle.fill")
                    .font(.title)
            }
            .buttonStyle(.plain)
            .accessibilityLabel(Text("Preview", bundle: .module))
            .accessibilityIdentifier("beats.preview.\(part.id)")

            VStack(alignment: .leading) {
                Text(verbatim: part.name)
                    .font(.dozens(.bodyRegular))
                Text(part.duration.formatted(.time(pattern: .minuteSecond)))
                    .font(.dozens(.bodySmall))
                    .monospacedDigit()
                    .foregroundStyle(.secondary)
            }

            Spacer()

            Button {
                stopPreview()
                model.select(part)
            } label: {
                Text("Record", bundle: .module)
            }
            .buttonStyle(.borderedProminent)
            .tint(.dozensRed)
            .accessibilityIdentifier("beats.record.\(part.id)")
        }
    }

    private func togglePreview(_ part: BeatPart) {
        if previewingPartID == part.id {
            stopPreview()
            return
        }
        preview.replaceCurrentItem(with: AVPlayerItem(url: part.audioURL))
        preview.play()
        previewingPartID = part.id
    }

    private func stopPreview() {
        preview.pause()
        previewingPartID = nil
    }
}
