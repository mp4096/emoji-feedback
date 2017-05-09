const buttons = document.querySelectorAll(".emoji-button");
for(var i = 0; i < buttons.length; i++) {
  buttons[i].addEventListener("webkitAnimationEnd", function() {
    // reset animation
    this.style.animationName = "";
  });
}

const thanksEl = document.querySelector("#thanks");
thanksEl.addEventListener("webkitAnimationEnd", function() {
  // reset animation
  thanksEl.style.animationName = "";
});

function post_feedback(fb) {
  var button = document.querySelector(".emoji-button." + fb);
  button.style.animationName = "pop";

  fetch("/feedback/" + fb, {method: "POST"}).then(function(response) {
    if (response.ok) {
      thanksEl.style.animationName = "woof";
    }
  }).catch(console.log);
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
