# 🎄 Day 11: Imagery from the North Pole

*Decked out in his signature red coat, Santa's eyes sparkle brighter than the Northern Star as he navigates through tall shelves packed with newly produced Christmas decorations for the season. Handcrafted glass balls, ornate stars, whimsical snowflakes, festive tinsel - you name it, they have it all.*

## ⭐ Task 1: Served on a silver platter

The time has come to decorate our surroundings! The elves are getting tired of working with just strings and numbers and bytes, and are in need of some fancy christmas ornaments on the computer screens.

![decoration](https://cch23.shuttleapp.rs/assets/decoration.png)

Download the image above and serve it as a static file so that a GET request to `/11/assets/decoration.png` responds with the image file and correct headers for MIME type (`Content-Type: image/png`) and response length (`Content-Length: ...`).

### 🔔 Tips

Web frameworks usually provide an easy way to serve files and automatically setting the type and length headers based on the file served.

- [Shuttle examples: Axum static files](https://github.com/shuttle-hq/shuttle-examples/tree/main/axum/static-files)
- [Shuttle examples: Actix Web static files](https://github.com/shuttle-hq/shuttle-examples/tree/main/actix-web/static-files)
- [Shuttle examples: Rocket static files](https://github.com/shuttle-hq/shuttle-examples/tree/main/rocket/static-files)
- [MDN Wed docs: Content-Type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type)
- [MDN Wed docs: MIME types](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)

### 💠 Example Input

```bash
curl -I -X GET http://localhost:8000/11/assets/decoration.png
```

### 💠 Example Output

```text
HTTP/1.1 200 OK
content-type: image/png
content-length: 787297
...
```

---

## 🎁 Task 2: Bull mode activated (200 bonus points)

Add a POST endpoint `/11/red_pixels`, that takes in a PNG image in the `image` field of a multipart POST request, and returns the number of pixels in the image that are perceived as "magical red" when viewed with Santa's night vision goggles.
The goggles considers a pixel "magical red" if the color values of the pixel fulfill the formula `red > blue + green`.

### 💠 Example

```bash
curl -X POST http://localhost:8000/11/red_pixels \
  -H 'Content-Type: multipart/form-data' \
  -F 'image=@decoration.png' # the image from Task 1

73034
```

---

Authors: [orhun](https://github.com/orhun), [jonaro00](https://github.com/jonaro00)
