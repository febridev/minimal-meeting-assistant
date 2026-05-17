import Foundation
import ScreenCaptureKit
import AVFoundation

// MARK: - Rust Interface

@_silgen_name("rust_add_data")
func rust_add_data(_ buffer: UnsafeRawPointer, _ data: UnsafePointer<Float>, _ len: Int)

// MARK: - Capture Manager

class CaptureManager: NSObject, SCStreamDelegate, SCContentSharingPickerObserver, SCStreamOutput {
    static let shared = CaptureManager()
    
    private var stream: SCStream?
    private var audioBuffer: UnsafeRawPointer?
    private let queue = DispatchQueue(label: "com.meeting.assistant.audio")
    
    func start(audio_buffer: UnsafeRawPointer) {
        self.audioBuffer = audio_buffer
        SCContentSharingPicker.shared.add(self)
        SCContentSharingPicker.shared.isActive = true
        SCContentSharingPicker.shared.present()
    }
    
    func stop() {
        stream?.stopCapture()
        stream = nil
        audioBuffer = nil
        SCContentSharingPicker.shared.isActive = false
    }

    // MARK: - SCContentSharingPickerObserver
    
    func contentSharingPicker(_ picker: SCContentSharingPicker, didUpdateWith filter: SCContentFilter, for stream: SCStream?) {
        let config = SCStreamConfiguration()
        config.capturesAudio = true
        config.sampleRate = 48000
        config.channelCount = 1
        
        let newStream = SCStream(filter: filter, configuration: config, delegate: self)
        do {
            try newStream.addStreamOutput(self, type: .audio, sampleHandlerQueue: queue)
            newStream.startCapture { error in
                if let error = error {
                    print("Start capture error: \(error)")
                }
            }
            self.stream = newStream
        } catch {
            print("Setup stream error: \(error)")
        }
    }

    func contentSharingPicker(_ picker: SCContentSharingPicker, didCancelFor stream: SCStream?) {
        print("Picker cancelled")
    }
    
    func contentSharingPickerStartDidFailWithError(_ error: Error) {
        print("Picker failed: \(error)")
    }

    // MARK: - SCStreamOutput
    
    func stream(_ stream: SCStream, didOutputSampleBuffer sampleBuffer: CMSampleBuffer, of type: SCStreamOutputType) {
        guard type == .audio, let audioBuffer = audioBuffer else { return }
        
        guard let blockBuffer = CMSampleBufferGetDataBuffer(sampleBuffer) else { return }
        let length = CMBlockBufferGetDataLength(blockBuffer)
        
        var dataPointer: UnsafeMutablePointer<Int8>?
        CMBlockBufferGetDataPointer(blockBuffer, atOffset: 0, lengthAtOffsetOut: nil, totalLengthOut: nil, dataPointerOut: &dataPointer)
        
        if let data = dataPointer {
            let floatPtr = data.withMemoryRebound(to: Float.self, capacity: length / 4) { $0 }
            rust_add_data(audioBuffer, floatPtr, length / 4)
        }
    }
}

// MARK: - C Exports

@_cdecl("start_capture")
public func start_capture(audio_buffer: UnsafeRawPointer) {
    CaptureManager.shared.start(audio_buffer: audio_buffer)
}

@_cdecl("stop_capture")
public func stop_capture() {
    CaptureManager.shared.stop()
}
