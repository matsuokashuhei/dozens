#if os(iOS)
import AVFoundation

/// フロント／バックカメラとマイクで録画する。
final class CameraCapture: NSObject, VideoCapture, AVCaptureFileOutputRecordingDelegate {
    let session = AVCaptureSession()
    private let output = AVCaptureMovieFileOutput()
    private var finished: CheckedContinuation<URL?, Never>?
    private(set) var position: AVCaptureDevice.Position = .front

    static var isAvailable: Bool {
        AVCaptureDevice.default(.builtInWideAngleCamera, for: .video, position: .unspecified) != nil
    }

    func configure(position: AVCaptureDevice.Position) async -> Bool {
        guard await AVCaptureDevice.requestAccess(for: .video),
              await AVCaptureDevice.requestAccess(for: .audio),
              let camera = AVCaptureDevice.default(.builtInWideAngleCamera, for: .video, position: position),
              let cameraInput = try? AVCaptureDeviceInput(device: camera)
        else { return false }

        session.beginConfiguration()
        session.inputs.forEach(session.removeInput)
        session.sessionPreset = .high
        if session.canAddInput(cameraInput) { session.addInput(cameraInput) }
        if let mic = AVCaptureDevice.default(for: .audio),
           let micInput = try? AVCaptureDeviceInput(device: mic),
           session.canAddInput(micInput) {
            session.addInput(micInput)
        }
        if !session.outputs.contains(output), session.canAddOutput(output) { session.addOutput(output) }
        session.commitConfiguration()
        self.position = position

        let box = SessionBox(session: session)
        await Task.detached { if !box.session.isRunning { box.session.startRunning() } }.value
        return true
    }

    func shutdown() {
        let box = SessionBox(session: session)
        Task.detached { box.session.stopRunning() }
    }

    func start() {
        let url = URL.temporaryDirectory.appending(path: "take-\(UUID().uuidString).mov")
        output.startRecording(to: url, recordingDelegate: self)
    }

    func stop() async -> URL? {
        guard output.isRecording else { return nil }
        return await withCheckedContinuation { continuation in
            finished = continuation
            output.stopRecording()
        }
    }

    nonisolated func fileOutput(
        _ output: AVCaptureFileOutput,
        didFinishRecordingTo outputFileURL: URL,
        from connections: [AVCaptureConnection],
        error: (any Error)?
    ) {
        let exists = FileManager.default.fileExists(atPath: outputFileURL.path(percentEncoded: false))
        Task { @MainActor in
            self.finished?.resume(returning: exists ? outputFileURL : nil)
            self.finished = nil
        }
    }
}
/// `startRunning()` / `stopRunning()` はブロッキングなのでメインスレッド外で呼ぶ。
/// 古い SDK では `AVCaptureSession` が `Sendable` でないため包んで渡す。
nonisolated private struct SessionBox: @unchecked Sendable {
    let session: AVCaptureSession
}
#endif
