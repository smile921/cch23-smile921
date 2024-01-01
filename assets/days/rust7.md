# 🎄 Day 7: GET Santa some cookies

*At Santa's base near the North Pole (64 km away to be precise), the scent of freshly baked cookies fills the air, a sign that Christmas is near. Santa, however, has forgotten the encoding method that was used to hide his favorite cookie recipe in web browsers around the world. He needs this super-secret recipe (based on his grandfather's recipe made in 1964!) to fuel his late-night toy-making sessions.*

## ⭐ Task 1: Based encoding, 64th edition

Santa's secret cookie recipe is hidden in an encoded message,
and he's looking to you for help. He's sending a GET request to your
server with a `Cookie` header.

What you need to do is parse the Cookie header, decode the value in the *recipe* field, and return it.

Make an endpoint `/7/decode` that extracts the `Cookie` HTTP header.
The header in the request will look something like this:

```text
Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
```

After decoding the recipe value to bytes, convert it to a string and return it as the response to the GET request.

### 🔔 Tips

*"Which encoding?"* you might ask. Look around this page and the links below to get some hints!

- [Binary-to-text encoding standards](https://en.wikipedia.org/wiki/Binary-to-text_encoding#Encoding_standards)
- [Crates.io: Encoding](https://crates.io/categories/encoding?sort=downloads)
- [MDN Web docs: Cookie](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie)

### 💠 Example

```bash
curl http://localhost:8000/7/decode \
  -H 'Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ=='

{"flour":100,"chocolate chips":20}
```

---

## 🎁 Task 2: The secret cookie recipe (120 bonus points)

Now that the recipe is decoded, Santa can get back to baking cookies! Santa is not sure, however, if he has enough of each ingredient to bake a cookie for every elf.

The same type of request as in Task 1 will be sent to a new endpoint, `/7/bake`, but this time Santa needs your help to calculate the amount of cookies he can bake with the ingredients he has in the pantry.

After decoding, parse the bytes as a JSON object (shape and keys can be seen in the example below) and use that to calculate how many cookies can be baked with the provided recipe and ingredients. Additionally, return the amount of ingredients that would remain in the pantry after the cookies have been baked.

### 💠 Example Input

```bash
curl http://localhost:8000/7/bake \
  -H 'Cookie: recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319'
```

After decoding, the recipe above will look like this JSON object:

```text
{
  "recipe": {
    "flour": 95,
    "sugar": 50,
    "butter": 30,
    "baking powder": 10,
    "chocolate chips": 50
  },
  "pantry": {
    "flour": 385,
    "sugar": 507,
    "butter": 2122,
    "baking powder": 865,
    "chocolate chips": 457
  }
}
```

### 💠 Example Output

```text
{
  "cookies": 4,
  "pantry": {
    "flour": 5,
    "sugar": 307,
    "butter": 2002,
    "baking powder": 825,
    "chocolate chips": 257
  }
}
```

Explanation: The recipe represents the required ingredients to make one cookie. After baking 4 cookies, we have 5 units of flour left and can't bake any more.

---

## 🎁 Task 3: Questionable cookie recipes (100 bonus points)

Some mischievous elves have now found your endpoint, and are trying their "innovative" cookie recipes on it, without even knowing what ingredients are available in the pantry!

Update the endpoint from Task 2 so that any set of ingredients can be present in the recipe and the pantry, respectively.

The number of cookies in the answer will always be finite.

### 💠 Example
 
```text
curl http://localhost:8000/7/bake \
  -H 'Cookie: recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ=='

{
  "cookies": 0,
  "pantry": {
    "cobblestone": 64,
    "stick": 4
  }
}
```
