import DesignSystem
import Domain
import SwiftUI

struct RecorderView: View {
    private let onFinish: (URL) -> Void
    private let onCancel: () -> Void
    @State private var model: RecorderModel
    @State private var recording: Task<Void, Never>?
    #if os(iOS)
    @State private var camera: CameraCapture?
    #endif

    init(part: BeatPart, onFinish: @escaping (URL) -> Void, onCancel: @escaping () -> Void) {
        self.onFinish = onFinish
        self.onCancel = onCancel
        #if os(iOS)
        let camera = CameraCapture.isAvailable ? CameraCapture() : nil
        _camera = State(initialValue: camera)
        _model = State(initialValue: RecorderModel(part: part, capture: camera ?? SampleVideoCapture()))
        #else
        _model = State(initialValue: RecorderModel(part: part, capture: SampleVideoCapture()))
        #endif
    }

    var body: some View {
        ZStack {
            preview
                .ignoresSafeArea()
            if case .countingDown(let count) = model.phase {
                Text(verbatim: "\(count)")
                    .font(.system(size: 120, weight: .heavy, design: .rounded))
                    .foregroundStyle(.white)
                    .contentTransition(.numericText(countsDown: true))
                    .accessibilityIdentifier("recorder.countdown")
            }
            VStack {
                Spacer()
                controls
            }
            .padding()
        }
        .navigationTitle(Text(verbatim: model.part.name))
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar(.hidden, for: .tabBar)
        #endif
        .toolbar {
            ToolbarItem(placement: .cancellationAction) {
                Button {
                    recording?.cancel()
                    onCancel()
                } label: {
                    Text("Cancel", bundle: .module)
                }
                .accessibilityIdentifier("recorder.cancel")
            }
        }
        .task {
            #if os(iOS)
            _ = await camera?.configure(position: .front)
            #endif
        }
        .onDisappear {
            recording?.cancel()
            model.beatPlayer.pause()
            #if os(iOS)
            camera?.shutdown()
            #endif
        }
    }

    @ViewBuilder
    private var preview: some View {
        #if os(iOS)
        if let camera {
            CameraPreview(session: camera.session)
        } else {
            unavailable
        }
        #else
        unavailable
        #endif
    }

    private var unavailable: some View {
        ZStack {
            Color.black
            Text("Camera unavailable. A sample video will be used.", bundle: .module)
                .font(.dozens(.bodySmall))
                .foregroundStyle(.white.opacity(0.7))
                .multilineTextAlignment(.center)
                .padding()
        }
    }

    private var controls: some View {
        VStack(spacing: 16) {
            ProgressView(value: model.progress)
                .tint(.dozensRed)
                .accessibilityIdentifier("recorder.progress")
            Text(verbatim: "\(model.elapsed.formatted(.time(pattern: .minuteSecond))) / \(model.part.duration.formatted(.time(pattern: .minuteSecond)))")
                .font(.dozens(.bodySmall))
                .monospacedDigit()
                .foregroundStyle(.white)
                .accessibilityIdentifier("recorder.time")
            HStack {
                Spacer()
                recordButton
                Spacer()
            }
            .overlay(alignment: .trailing) { switchCameraButton }
        }
        .padding()
        .dozensGlassEffect(in: .rect(cornerRadius: 24))
    }

    private var recordButton: some View {
        Button {
            if model.phase == .ready {
                recording = Task {
                    if let url = await model.record() { onFinish(url) }
                }
            } else {
                model.stop()
            }
        } label: {
            ZStack {
                Circle()
                    .stroke(.white, lineWidth: 4)
                    .frame(width: 72, height: 72)
                RoundedRectangle(cornerRadius: model.phase == .ready ? 30 : 6)
                    .fill(Color.dozensRed)
                    .frame(width: model.phase == .ready ? 60 : 30, height: model.phase == .ready ? 60 : 30)
            }
            .animation(.snappy, value: model.phase)
        }
        .buttonStyle(.plain)
        .disabled(model.phase == .finished)
        .accessibilityLabel(model.phase == .ready ? Text("Record", bundle: .module) : Text("Stop", bundle: .module))
        .accessibilityIdentifier("recorder.record")
    }

    @ViewBuilder
    private var switchCameraButton: some View {
        #if os(iOS)
        if let camera {
            Button {
                Task { _ = await camera.configure(position: camera.position == .front ? .back : .front) }
            } label: {
                Image(systemName: "arrow.triangle.2.circlepath.camera")
                    .font(.title2)
                    .foregroundStyle(.white)
            }
            .disabled(model.phase != .ready)
            .accessibilityLabel(Text("Switch Camera", bundle: .module))
        }
        #endif
    }
}
