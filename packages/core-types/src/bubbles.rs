/// Check if the event bubbles
///
/// todo: this should not be in this crate, but this crate is a "root" crate and
/// has zero-deps, meaning it gets compiled before anything else.
///
/// This function being here means we can use it in the interpreter without pulling in dioxus-html,
/// drastically shortening the crate graph and thus compile times
///
/// The real solution to this problem is that events need to mark themselves as "bubbling" or "not bubbling"
/// in their definition, which gets passed as part of the mutations.
pub fn event_bubbles(evt: &str) -> bool {
    match evt {
        "copy" => true,
        "cut" => true,
        "paste" => true,
        "compositionend" => true,
        "compositionstart" => true,
        "compositionupdate" => true,
        "keydown" => true,
        "keypress" => true,
        "keyup" => true,
        "focus" => false,
        "focusout" => true,
        "focusin" => true,
        "blur" => false,
        "change" => true,
        "input" => true,
        "invalid" => true,
        "reset" => true,
        "submit" => true,
        "click" => true,
        "contextmenu" => true,
        "doubleclick" => true,
        "dblclick" => true,
        "drag" => true,
        "dragend" => true,
        "dragenter" => false,
        "dragexit" => false,
        "dragleave" => true,
        "dragover" => true,
        "dragstart" => true,
        "drop" => true,
        "mousedown" => true,
        "mouseenter" => false,
        "mouseleave" => false,
        "mousemove" => true,
        "mouseout" => true,
        "scroll" => false,
        "mouseover" => true,
        "mouseup" => true,
        "pointerdown" => true,
        "pointermove" => true,
        "pointerup" => true,
        "pointercancel" => true,
        "gotpointercapture" => true,
        "lostpointercapture" => true,
        "pointerenter" => false,
        "pointerleave" => false,
        "pointerover" => true,
        "pointerout" => true,
        "select" => true,
        "touchcancel" => true,
        "touchend" => true,
        "touchmove" => true,
        "touchstart" => true,
        "wheel" => true,
        "abort" => false,
        "canplay" => false,
        "canplaythrough" => false,
        "durationchange" => false,
        "emptied" => false,
        "encrypted" => true,
        "ended" => false,
        "error" => false,
        "loadeddata" => false,
        "loadedmetadata" => false,
        "loadstart" => false,
        "load" => false,
        "pause" => false,
        "play" => false,
        "playing" => false,
        "progress" => false,
        "ratechange" => false,
        "resize" => false,
        "seeked" => false,
        "seeking" => false,
        "stalled" => false,
        "suspend" => false,
        "timeupdate" => false,
        "volumechange" => false,
        "waiting" => false,
        "animationstart" => true,
        "animationend" => true,
        "animationiteration" => true,
        "transitionend" => true,
        "toggle" => true,
        "mounted" => false,
        _ => true,
    }
}