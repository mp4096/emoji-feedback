function post_feedback(fb) {
  fetch("/feedback/" + fb, {method: "POST"}).catch(console.log);
}
