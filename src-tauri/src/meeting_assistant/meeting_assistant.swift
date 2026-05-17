public func add_data(_ buffer: AudioBufferRef, _ data: UnsafeBufferPointer<Float>) {
    __swift_bridge__$add_data(buffer.ptr, data.toFfiSlice())
}
@_cdecl("__swift_bridge__$start_capture")
func __swift_bridge__start_capture (_ audio_buffer: UnsafeMutableRawPointer) {
    start_capture(audio_buffer: AudioBuffer(ptr: audio_buffer))
}

@_cdecl("__swift_bridge__$stop_capture")
func __swift_bridge__stop_capture () {
    stop_capture()
}


public class AudioBuffer: AudioBufferRefMut {
    var isOwned: Bool = true

    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }

    deinit {
        if isOwned {
            __swift_bridge__$AudioBuffer$_free(ptr)
        }
    }
}
public class AudioBufferRefMut: AudioBufferRef {
    public override init(ptr: UnsafeMutableRawPointer) {
        super.init(ptr: ptr)
    }
}
public class AudioBufferRef {
    var ptr: UnsafeMutableRawPointer

    public init(ptr: UnsafeMutableRawPointer) {
        self.ptr = ptr
    }
}
extension AudioBuffer: Vectorizable {
    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        __swift_bridge__$Vec_AudioBuffer$new()
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        __swift_bridge__$Vec_AudioBuffer$drop(vecPtr)
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: AudioBuffer) {
        __swift_bridge__$Vec_AudioBuffer$push(vecPtr, {value.isOwned = false; return value.ptr;}())
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> Optional<Self> {
        let pointer = __swift_bridge__$Vec_AudioBuffer$pop(vecPtr)
        if pointer == nil {
            return nil
        } else {
            return (AudioBuffer(ptr: pointer!) as! Self)
        }
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AudioBufferRef> {
        let pointer = __swift_bridge__$Vec_AudioBuffer$get(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AudioBufferRef(ptr: pointer!)
        }
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> Optional<AudioBufferRefMut> {
        let pointer = __swift_bridge__$Vec_AudioBuffer$get_mut(vecPtr, index)
        if pointer == nil {
            return nil
        } else {
            return AudioBufferRefMut(ptr: pointer!)
        }
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<AudioBufferRef> {
        UnsafePointer<AudioBufferRef>(OpaquePointer(__swift_bridge__$Vec_AudioBuffer$as_ptr(vecPtr)))
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        __swift_bridge__$Vec_AudioBuffer$len(vecPtr)
    }
}



