function post_feedback(val) {
  fetch("/feedback/" + val, {method: "POST"})
}
