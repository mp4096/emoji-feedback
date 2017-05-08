function post_feedback(fb) {
  fetch("/feedback/" + fb, {method: "POST"}).catch(console.log);
}

function is_fullscreen() {
  return document.fullscreenElement || // standards-compliant browsers
    document.webkitFullscreenElement || // Chrome, Opera, Edge, Safari
    document.mozFullScreenElement; // Firefox
}

function exit_fullscreen() {
  if (document.exitFullscreen) {
    document.exitFullscreen(); // standards-compliant browsers
  } else if (document.webkitExitFullscreen) {
    document.webkitExitFullscreen(); // Chrome, Opera, Edge, Safari
  } else if (document.mozCancelFullScreen) {
    document.mozCancelFullScreen(); // Firefox
  }
}

function request_fullscreen() {
  var el = document.body;

  if (el.requestFullscreen) {
    el.requestFullscreen(); // standards-compliant browsers
  } else if (el.webkitRequestFullScreen) {
    el.webkitRequestFullscreen(); // Chrome, Opera, Edge, Safari
  } else if (el.mozRequestFullScreen) {
    el.mozRequestFullScreen(); // Firefox
  }
}

function toggle_fullscreen() {
  if (is_fullscreen()) {
    exit_fullscreen();
  } else {
    request_fullscreen();
  }
}
