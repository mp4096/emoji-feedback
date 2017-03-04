function post_feedback(val) {
  fetch("./feedback", {
      method: "POST",
      body: val,
    })
}
