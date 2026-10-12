import DesignSystem
import Domain
import SwiftUI

/// 録音タブ。ビート選択 → 撮影 → 投稿 を `RecordModel.step` に従って切り替える。
public struct RecordView: View {
    @Bindable private var model: RecordModel

    public init(model: RecordModel) {
        self.model = model
    }

    public var body: some View {
        NavigationStack {
            content
                .background(.dozensBackground)
        }
    }

    @ViewBuilder
    private var content: some View {
        switch model.step {
        case .beats:
            BeatPickerView(model: model)
        case .recording(let part):
            RecorderView(
                part: part,
                onFinish: { model.finishRecording(videoURL: $0, part: part) },
                onCancel: model.discardRecording
            )
            .id(part.id)
        case .review(let draft):
            PublishView(model: model, draft: draft)
                .id(draft.videoURL)
        case .published:
            ContentUnavailableView {
                Label {
                    Text("Posted!", bundle: .module)
                } icon: {
                    Image(systemName: "checkmark.circle")
                }
            } description: {
                Text("Your rap will appear on the timeline.", bundle: .module)
            } actions: {
                Button {
                    model.start(hookUp: nil)
                } label: {
                    Text("Record Another", bundle: .module)
                }
                .dozensProminentButtonStyle()
                .accessibilityIdentifier("record.again")
            }
        }
    }
}
