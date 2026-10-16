const apiUrl =
  location.hostname === "shorter.dev"
    ? "https://api.shorter.dev"
    : "http://127.0.0.1:8080";

document.body.addEventListener("htmx:config:request", function (e) {
  e.detail.ctx.request.action = apiUrl + e.detail.ctx.request.action;
});

function example(q) {
  const input = document.getElementById("q");
  input.value = q;
  htmx.trigger(input, "input");
}
