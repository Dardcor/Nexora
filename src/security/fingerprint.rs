/// Anti-fingerprinting scripts injected into every page prior to DOM construction.
/// Protects users against cross-site hardware & API fingerprinting techniques.
pub struct AntiFingerprint;

impl AntiFingerprint {
    /// Generates the strict defense script injected at document-start.
    pub fn get_stealth_injection_script() -> &'static str {
        r#"
        (function() {
            'use strict';
            if (window.__nexora_protected__) return;
            window.__nexora_protected__ = true;

            // 1. Prevent Webdriver / Automation detection
            Object.defineProperty(navigator, 'webdriver', {
                get: () => undefined,
                configurable: true
            });

            // 2. Standardize hardware concurrency and memory reporting
            Object.defineProperty(navigator, 'hardwareConcurrency', {
                get: () => 4,
                configurable: true
            });
            Object.defineProperty(navigator, 'deviceMemory', {
                get: () => 8,
                configurable: true
            });

            // 3. Canvas Fingerprint Protection (Injects subtle, invisible non-deterministic noise)
            const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
            HTMLCanvasElement.prototype.toDataURL = function(type) {
                const ctx = this.getContext('2d');
                if (ctx) {
                    try {
                        const width = this.width;
                        const height = this.height;
                        if (width > 0 && height > 0) {
                            const imgData = ctx.getImageData(0, 0, 1, 1);
                            // Micro-shift least significant bit of 1 pixel
                            imgData.data[0] = (imgData.data[0] + 1) % 256;
                            ctx.putImageData(imgData, 0, 0);
                        }
                    } catch(e) {}
                }
                return origToDataURL.apply(this, arguments);
            };

            const origGetImageData = CanvasRenderingContext2D.prototype.getImageData;
            CanvasRenderingContext2D.prototype.getImageData = function() {
                const res = origGetImageData.apply(this, arguments);
                if (res && res.data && res.data.length > 4) {
                    // Inject subtle noise on pixel 0
                    res.data[0] = (res.data[0] ^ 1);
                }
                return res;
            };

            // 4. WebGL Vendor & Renderer Masking
            const origGetParameter = WebGLRenderingContext.prototype.getParameter;
            WebGLRenderingContext.prototype.getParameter = function(param) {
                // UNMASKED_VENDOR_WEBGL (0x9245)
                if (param === 0x9245) {
                    return 'Intel Inc.';
                }
                // UNMASKED_RENDERER_WEBGL (0x9246)
                if (param === 0x9246) {
                    return 'Intel(R) Iris(R) Xe Graphics';
                }
                return origGetParameter.apply(this, arguments);
            };

            // 5. Disable Battery API tracking
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

            // 6. AudioContext Fingerprint Noise
            if (window.AudioBuffer) {
                const origCopyFromChannel = AudioBuffer.prototype.copyFromChannel;
                AudioBuffer.prototype.copyFromChannel = function(dest, channelNumber, startInChannel) {
                    origCopyFromChannel.apply(this, arguments);
                    if (dest && dest.length > 0) {
                        dest[0] = dest[0] + 0.0000001;
                    }
                };
            }
        })();
        "#
    }
}
