pub struct AntiFingerprint;

impl AntiFingerprint {
    pub fn get_stealth_injection_script() -> &'static str {
        r#"
        (function() {
            'use strict';
            if (window.__nexora_shield_active__) return;
            window.__nexora_shield_active__ = true;

            try {
                const lockStyle = document.createElement('style');
                lockStyle.id = '__nexora_viewport_lock__';
                lockStyle.textContent = 'html, body { overscroll-behavior: none !important; overscroll-behavior-y: none !important; overscroll-behavior-x: none !important; } html { scrollbar-gutter: stable; }';
                if (document.head) {
                    document.head.appendChild(lockStyle);
                } else {
                    document.addEventListener('DOMContentLoaded', function() {
                        if (document.head && !document.getElementById('__nexora_viewport_lock__')) {
                            document.head.appendChild(lockStyle);
                        }
                    });
                }
            } catch(e) {}

            if (window.AudioBuffer && AudioBuffer.prototype.copyFromChannel) {
                const origCopy = AudioBuffer.prototype.copyFromChannel;
                AudioBuffer.prototype.copyFromChannel = function(dest, channelNumber, startInChannel) {
                    origCopy.apply(this, arguments);
                    if (dest && dest.length > 0) {
                        dest[0] = dest[0] + 0.00000001;
                    }
                };
            }

            if (navigator.getBattery) {
                navigator.getBattery = function() {
                    return Promise.resolve({
                        charging: true,
                        chargingTime: 0,
                        dischargingTime: Infinity,
                        level: 1.0,
                        onchargingchange: null,
                        onchargingtimechange: null,
                        ondischargingtimechange: null,
                        onlevelchange: null
                    });
                };
            }

            window.addEventListener('keydown', function(e) {
                if (!window.ipc) return;
                const key = (e.key || '').toLowerCase();
                let action = null;

                if (e.key === 'F5' && !e.ctrlKey && !e.altKey) {
                    action = 'reload';
                } else if (e.key === 'F11' && !e.ctrlKey && !e.altKey) {
                    action = 'fullscreen';
                } else if (e.ctrlKey && !e.altKey && (e.key === '+' || e.key === '=')) {
                    action = 'zoom_in';
                } else if (e.ctrlKey && e.shiftKey && !e.altKey) {
                    if (key === 'n') action = 'new_incognito_window';
                    else if (key === 't') action = 'reopen_closed_tab';
                    else if (key === 'o') action = 'bookmarks';
                } else if (e.ctrlKey && !e.shiftKey && !e.altKey) {
                    if (key === 't') action = 'new_tab';
                    else if (key === 'n') action = 'new_window';
                    else if (key === 'w') action = 'close_tab';
                    else if (key === 'l') action = 'focus_omnibox';
                    else if (key === 'r') action = 'reload';
                    else if (key === 'h') action = 'history';
                    else if (key === 'j') action = 'downloads';
                    else if (key === 'f') action = 'find_in_page';
                    else if (key === 'p') action = 'print';
                    else if (e.key === '-') action = 'zoom_out';
                    else if (e.key === '0') action = 'zoom_reset';
                } else if (e.altKey && !e.ctrlKey && !e.shiftKey) {
                    if (e.key === 'ArrowLeft') action = 'go_back';
                    else if (e.key === 'ArrowRight') action = 'go_forward';
                    else if (key === 'r') action = 'reader_mode';
                    else if (key === 'f' || key === 'e') action = 'open_more_menu';
                }

                if (action) {
                    e.preventDefault();
                    e.stopPropagation();
                    window.ipc.postMessage(JSON.stringify({
                        type: 'shortcut',
                        action: action
                    }));
                }
            }, true);
        })();
        "#
    }
}
