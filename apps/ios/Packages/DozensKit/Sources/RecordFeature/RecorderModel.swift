import AVFoundation
import Domain
import Foundation
import Observation

/// カウントインのあとビートを流しながら、パートの長さだけ撮影する。
@Observable
final class RecorderModel {
    enum Phase: Equatable {
        case ready
        case countingDown(Int)
        case recording
        case finished
    }

    private(set) var phase: Phase = .ready
    private(set) var elapsed: Duration = .zero
    let part: BeatPart
    let capture: any VideoCapture
    let beatPlayer = AVPlayer()

    private let countInBeat: Duration
    private var stopRequested = false

    init(part: BeatPart, capture: any VideoCapture, countInBeat: Duration = .seconds(1)) {
        self.part = part
        self.capture = capture
        self.countInBeat = countInBeat
    }

    var progress: Double {
        min(1, elapsed / part.duration)
    }

    /// 撮影を最後まで行い、録画ファイルを返す。カウントイン中に止めた場合は `nil`。
    func record() async -> URL? {
        guard phase == .ready else { return nil }
        stopRequested = false

        for count in stride(from: 3, through: 1, by: -1) {
            phase = .countingDown(count)
            try? await Task.sleep(for: countInBeat)
            if stopRequested || Task.isCancelled {
                phase = .ready
                return nil
            }
        }

        phase = .recording
        elapsed = .zero
        capture.start()
        beatPlayer.replaceCurrentItem(with: AVPlayerItem(url: part.audioURL))
        beatPlayer.play()

        let clock = ContinuousClock()
        let startedAt = clock.now
        while elapsed < part.duration, !stopRequested, !Task.isCancelled {
            try? await Task.sleep(for: .milliseconds(50))
            elapsed = clock.now - startedAt
        }

        beatPlayer.pause()
        let url = await capture.stop()
        guard !Task.isCancelled else {
            phase = .ready
            return nil
        }
        phase = .finished
        return url
    }

    /// カウントイン中なら中止、撮影中ならそこで撮り終える。
    func stop() {
        stopRequested = true
    }
}
