function post_feedback(val) {
  fetch('/feedback', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      feedback_value: val,
    }),
  })
}
