public func add_data(_ buffer: AudioBufferRef, _ data: UnsafeBufferPointer<Float>) {
    __swift_bridge__$add_data(buffer.ptr, data.toFfiSlice())
}
@_cdecl("__swift_bridge__$start_capture")
func __swift_bridge__start_capture (_ audio_buffer: UnsafeMutableRawPointer) {
    start_capture(audio_buffer: AudioBufferRef(ptr: audio_buffer))
}

@_cdecl("__swift_bridge__$stop_capture")
func __swift_bridge__stop_capture () {
    stop_capture()
}





