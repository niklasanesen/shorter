function example(q) {
  const input = document.getElementById("q");
  input.value = q;
  htmx.trigger(input, "input");
}

const apiUrl =
  location.hostname === "shorter.dev"
    ? "https://5mmnbausmvkvlgyylpztxdkbsu0wnyyf.lambda-url.eu-west-1.on.aws"
    : "http://127.0.0.1:8080";

htmx.on("htmx:config:request", (e) => {
  e.detail.ctx.request.action = apiUrl + e.detail.ctx.request.action;
});

htmx.config.mode = "cors";
