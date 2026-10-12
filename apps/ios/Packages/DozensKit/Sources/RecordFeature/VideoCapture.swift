import Foundation

/// 撮影の開始と終了。終了時に録画ファイルの URL を返す。
protocol VideoCapture: AnyObject {
    func start()
    func stop() async -> URL?
}

/// カメラが使えない環境（シミュレータなど）で、同梱のサンプル動画を撮影結果として返す。
final class SampleVideoCapture: VideoCapture {
    func start() {}

    func stop() async -> URL? {
        guard let sample = Bundle.module.url(forResource: "sample-take", withExtension: "mp4") else { return nil }
        let url = URL.temporaryDirectory.appending(path: "take-\(UUID().uuidString).mp4")
        do {
            try FileManager.default.copyItem(at: sample, to: url)
            return url
        } catch {
            return nil
        }
    }
}
