import AVFoundation
import Application
import Domain
import Observation

/// サイファーのラップを投稿順に連続再生する。最後まで再生したら先頭に戻る。
@Observable
public final class CypherPlayerModel {
    public private(set) var cypher: Cypher?
    public private(set) var error: TimelineError?
    /// 表示・再生中のラップ。横スクロールの位置と双方向に同期する。
    public var currentRapID: Rap.ID?

    public let player = AVPlayer()

    private let cypherID: Cypher.ID
    private let startRapID: Rap.ID
    private let reader: TimelineReader

    public init(cypherID: Cypher.ID, startRapID: Rap.ID, reader: TimelineReader) {
        self.cypherID = cypherID
        self.startRapID = startRapID
        self.reader = reader
    }

    public var raps: [Rap] { cypher?.raps ?? [] }

    public var currentRap: Rap? {
        raps.first { $0.id == currentRapID }
    }

    public func load() async {
        do {
            let cypher = try await reader.cypher(id: cypherID)
            self.cypher = cypher
            error = nil
            currentRapID = cypher.raps.contains { $0.id == startRapID } ? startRapID : cypher.raps.first?.id
        } catch {
            self.error = error
        }
    }

    public func advance() {
        guard let index = raps.firstIndex(where: { $0.id == currentRapID }) else { return }
        currentRapID = raps[(index + 1) % raps.count].id
    }

    /// 現在のラップを再生し、終わったら次へ進む。`currentRapID` が変わったらキャンセルされる前提。
    public func playCurrentRap() async {
        guard let rap = currentRap else { return }
        let item = AVPlayerItem(url: rap.videoURL)
        player.replaceCurrentItem(with: item)
        player.play()
        let ends = NotificationCenter.default.notifications(named: AVPlayerItem.didPlayToEndTimeNotification, object: item)
        for await _ in ends {
            if raps.count > 1 {
                advance()
                return
            }
            await player.seek(to: .zero)
            player.play()
        }
    }

    public func togglePlayback() {
        if player.timeControlStatus == .paused {
            player.play()
        } else {
            player.pause()
        }
    }
}
