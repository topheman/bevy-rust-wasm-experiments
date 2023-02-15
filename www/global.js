/**
 * Exposed function to rust source code via WebAssembly through wasm-bindgen
 *
 * Those functions will be called from rust source code, search for `#[wasm_bindgen]` directives
 */

function resize_canvas(width, height) {
  console.log(width, height);
  const canvas = document.querySelector('canvas');
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  canvas.width = width * window.devicePixelRatio;
  canvas.height = height * window.devicePixelRatio;
}

// vitejs only accepts <script type="module"/> so we must expose bindings on global scope so they are accessible
window.resize_canvas = resize_canvas;

((win) => {
  if (/mobile/i.test(navigator.userAgent)) {
    document.body.classList.add('is-mobile');
  }
  /**
   * screen.orientation not available on safari ios, so we track it ourselves
   */
  let _isPortrait, _documentVisibilityState;
  // for performance reasons, we want to avoid querying directly the DOM and prefer caching
  function updateInfos() {
    _isPortrait = /mobile/i.test(navigator.userAgent) ? window.innerHeight > window.innerWidth : true;
    _documentVisibilityState = document.visibilityState;
  }
  updateInfos();
  let _isPortraitTimer = setInterval(updateInfos, 100);

  win.is_stopped = function () {
    return !_isPortrait || _documentVisibilityState === 'hidden';
  }

  /**
   * Debugging function that should only be used in development
   */
  win.debug_is_stopped = function (isStopped) {
    clearInterval(_isPortraitTimer);
    _isPortrait = Boolean(isStopped);
  }
})(window)

/**
 * Track orientation
 */
const _orientation = {
  x: 0,
  y: 0,
}

/**
 * Expose orientation to rust via wasm
 */
window.get_orientation_x = function () {
  return _orientation.x;
}
window.get_orientation_y = function () {
  return _orientation.y;
}

function onDeviceOrientation(event) {
  _orientation.x = event.gamma / 20;
  _orientation.y = -event.beta / 20;
}

const requestAccessAsync = async () => {
  if (typeof DeviceOrientationEvent === "undefined") {
    console.log("Device orientation event is not supported by your browser");
    return false;
  }

  if (
    DeviceOrientationEvent.requestPermission &&
    typeof DeviceMotionEvent.requestPermission === "function"
  ) {
    let permission;
    try {
      permission = await DeviceOrientationEvent.requestPermission();
    } catch (err) {
      console.error(err);
      return false;
    }
    if (permission !== "granted") {
      console.error("Request to access the device orientation was rejected", { permission });
      return false;
    }
  }

  window.addEventListener("deviceorientation", onDeviceOrientation);

  return true;
};

window.addEventListener('click', () => {
  requestAccessAsync()
}, { once: true })

console.log('window', window, 'document.body', document.body);
