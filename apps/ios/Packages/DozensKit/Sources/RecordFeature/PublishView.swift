import AVKit
import DesignSystem
import Domain
import SwiftUI

struct PublishView: View {
    @Bindable var model: RecordModel
    @State private var draft: RapDraft
    @State private var tagsText = ""
    @State private var player: AVPlayer

    init(model: RecordModel, draft: RapDraft) {
        self.model = model
        _draft = State(initialValue: draft)
        _player = State(initialValue: AVPlayer(url: draft.videoURL))
    }

    var body: some View {
        Form {
            Section {
                VideoPlayer(player: player)
                    .frame(height: 320)
                    .listRowInsets(EdgeInsets())
                    .accessibilityIdentifier("publish.preview")
            }
            if draft.hookUpRapID != nil {
                Section {
                    Label {
                        Text("Hook up to the cypher", bundle: .module)
                    } icon: {
                        Image(systemName: "link")
                    }
                }
            }
            Section {
                TextField(String(localized: "Lyrics", bundle: .module), text: $draft.lyrics, axis: .vertical)
                    .lineLimit(3...8)
                    .accessibilityIdentifier("publish.lyrics")
            } header: {
                Text("Lyrics", bundle: .module)
            }
            Section {
                TextField(String(localized: "Tags", bundle: .module), text: $tagsText)
                    .autocorrectionDisabled()
                    #if os(iOS)
                    .textInputAutocapitalization(.never)
                    #endif
                    .accessibilityIdentifier("publish.tags")
            } header: {
                Text("Tags", bundle: .module)
            } footer: {
                Text("Separate tags with spaces.", bundle: .module)
            }
            Section {
                Picker(selection: $draft.language) {
                    Text("Japanese", bundle: .module).tag("ja")
                    Text("English", bundle: .module).tag("en")
                } label: {
                    Text("Language", bundle: .module)
                }
            }
        }
        .scrollContentBackground(.hidden)
        .disabled(model.isPublishing)
        .navigationTitle(Text("Post", bundle: .module))
        .toolbar {
            ToolbarItem(placement: .cancellationAction) {
                Button {
                    player.pause()
                    model.discardRecording()
                } label: {
                    Text("Retake", bundle: .module)
                }
                .disabled(model.isPublishing)
                .accessibilityIdentifier("publish.retake")
            }
            ToolbarItem(placement: .confirmationAction) {
                if model.isPublishing {
                    ProgressView()
                } else {
                    Button {
                        player.pause()
                        var draft = draft
                        draft.tags = RecordModel.tags(from: tagsText)
                        Task { await model.publish(draft) }
                    } label: {
                        Text("Post", bundle: .module)
                    }
                    .accessibilityIdentifier("publish.post")
                }
            }
        }
        .alert(
            Text("Couldn't post the rap.", bundle: .module),
            isPresented: Binding(get: { model.error != nil }, set: { if !$0 { model.error = nil } })
        ) {
            Button {
            } label: {
                Text("OK", bundle: .module)
            }
        }
        .onAppear { player.play() }
        .onDisappear { player.pause() }
    }
}
