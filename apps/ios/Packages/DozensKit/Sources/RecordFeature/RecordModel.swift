import Application
import Domain
import Foundation
import Observation

/// 録音タブの画面遷移（ビート選択 → 撮影 → 投稿）を管理する。
@Observable
public final class RecordModel {
    public enum Step: Hashable {
        case beats
        case recording(BeatPart)
        case review(RapDraft)
        case published(Rap.ID)
    }

    /// 既存サイファーへのフックアップ。ビートはそのサイファーのものに固定される。
    public struct HookUp: Hashable, Sendable {
        public var rapID: Rap.ID
        public var beatID: Beat.ID

        public init(rapID: Rap.ID, beatID: Beat.ID) {
            self.rapID = rapID
            self.beatID = beatID
        }
    }

    public var step: Step = .beats
    public private(set) var beats: [Beat] = []
    public private(set) var isLoaded = false
    public private(set) var hookUp: HookUp?
    public private(set) var isPublishing = false
    public var error: RecordingError?

    private let service: RecordingService

    public init(service: RecordingService) {
        self.service = service
    }

    public func load() async {
        let hookUp = self.hookUp
        do {
            let beats: [Beat]
            if let hookUp {
                beats = [try await service.beat(id: hookUp.beatID)]
            } else {
                beats = try await service.beats()
            }
            guard hookUp == self.hookUp else { return }
            self.beats = beats
            error = nil
        } catch {
            guard hookUp == self.hookUp else { return }
            self.error = error
        }
        isLoaded = true
    }

    public func start(hookUp: HookUp?) {
        self.hookUp = hookUp
        beats = []
        isLoaded = false
        error = nil
        step = .beats
    }

    public func select(_ part: BeatPart) {
        step = .recording(part)
    }

    public func finishRecording(videoURL: URL, part: BeatPart) {
        step = .review(RapDraft(videoURL: videoURL, beatPartID: part.id, hookUpRapID: hookUp?.rapID))
    }

    public func discardRecording() {
        step = .beats
    }

    public func publish(_ draft: RapDraft) async {
        guard !isPublishing else { return }
        isPublishing = true
        defer { isPublishing = false }
        do {
            let id = try await service.publish(draft)
            hookUp = nil
            step = .published(id)
        } catch {
            self.error = error
        }
    }

    /// `#tag1 tag2, tag3` のような入力をタグ名の配列にする。
    public static func tags(from text: String) -> [String] {
        var seen = Set<String>()
        return text
            .split(whereSeparator: { $0.isWhitespace || $0 == "," || $0 == "、" })
            .map { String($0.drop(while: { $0 == "#" || $0 == "＃" })) }
            .filter { !$0.isEmpty && seen.insert($0).inserted }
    }
}
