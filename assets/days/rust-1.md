# 🎄 Day -1: Get your winter boots on!

Welcome to Shuttle's Christmas Code Hunt!

This challenge is a warmup challenge made to familiarize you with deploying your CCH23 project on Shuttle,
and it does not count towards your score.

---

## 🔔 Challenge layout

Every challenge is split into one or more *Core Tasks* (marked with ⭐) and one or more *Bonus Tasks* (marked with 🎁).

To **complete** a challenge, you only need to pass the tests for the *Core Tasks* ⭐.

*Bonus Tasks* 🎁 are harder to complete but give even more points. Their tests contain more edge cases and curveballs.

You can complete the *Core Tasks* ⭐ first (submit at the bottom of the page), and then work on *Bonus Tasks* 🎁.
Your highest scoring submission is the one that counts.

---

## ⭐ Task 1: Everything is OK

Deploy a minimal working web app to your CCH23 Shuttle project.

The root endpoint `/` should respond with a `200 OK` status code to GET requests.
Responding with a "Hello, world!" string, like the starter templates do, is enough to accomplish this.

### 🔔 Tips

Navigate to your CCH23 project folder. It should have code for a "Hello, world!" app.

In your project directory, you can use `cargo shuttle run` to test your Shuttle app locally (see below).

**More reading:**

- [Shuttle docs: Quick start](https://docs.shuttle.rs/getting-started/quick-start)
- [Shuttle docs: Local run](https://docs.shuttle.rs/getting-started/local-run)
- [Shuttle docs: Axum](https://docs.shuttle.rs/examples/axum)
- [Shuttle docs: Actix Web](https://docs.shuttle.rs/examples/actix)
- [Shuttle docs: Rocket](https://docs.shuttle.rs/examples/rocket)
- [Shuttle docs: Tower, Warp, Salvo, Poem, Thruster, Tide](https://docs.shuttle.rs/examples/other)
- [MDN web docs: 200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200)

### 💠 Example Input

Many tasks throughout CCH23 can easily be tested with a `curl` command.
You can use it when testing locally, and then verify that the deployed app works.

```bash
# On a local run with `cargo shuttle run`
curl -I -X GET http://localhost:8000/
```

### 💠 Example Output

```text
HTTP/1.1 200 OK
...
```

With that done, you can now get your first passing task!

You can at this point scroll to the bottom of this page to submit and see if your Hello World app is working,
or keep going with the bonus task.

---

## 🎁 Task 2: Fake error (0 bonus points)

For this bonus task, add an endpoint on `/-1/error` that responds with the status code `500 Internal Server Error` to GET requests.
The response body content does not matter.

### 🔔 Tips

- [Axum Responses](https://docs.rs/axum/latest/axum/response/index.html)
- [Actix Web Responses](https://actix.rs/docs/response)
- [Rocket Responses](https://rocket.rs/v0.5/guide/responses/)
- [MDN web docs: 500 Internal Server Error](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)

### 💠 Example Input

```bash
curl -I -X GET http://localhost:8000/-1/error
```

### 💠 Example Output

```text
HTTP/1.1 500 Internal Server Error
...
```
