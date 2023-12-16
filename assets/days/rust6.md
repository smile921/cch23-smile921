# 🎄 Day 6: Elf on a shelf

*It's that time of year when the elves hide on shelves to watch over the children of the world, reporting back to Santa on who's been naughty or nice. However, this year's reports have been mixed up with the rest of the letters to Santa, and the word "elf" is hidden throughout a mountain of text.*

## ⭐ Task 1: Never count on an elf

Elves are notorious for their hide-and-seek skills,
and now they've hidden themselves in strings of text!

Create an endpoint `/6` that takes a POST request with a raw string as input and count how many times the substring `"elf"` appears.

The output should be a JSON object containing the count of the string `"elf"`.

### 🔔 Tips 

- [Rust Primitive Type str](https://doc.rust-lang.org/std/primitive.str.html)
- [Rust String struct](https://doc.rust-lang.org/std/string/struct.String.html)

### 💠 Examples

```bash
curl -X POST http://localhost:8000/6 \
  -H 'Content-Type: text/plain' \
  -d 'The mischievous elf peeked out from behind the toy workshop,
      and another elf joined in the festive dance.
      Look, there is also an elf on that shelf!'

{"elf":4}
```

---

## 🎁 Task 2: Shelf under an elf? (200 bonus points)

Add two fields to the response that counts:

- The number of occurrences of the string `"elf on a shelf"` in a field with the same name.
- The number of shelves that don't have an elf on it. That is, the number of strings `"shelf"` that are not preceded by the string `"elf on a "`. Put this count in the field `"shelf with no elf on it"`.

### 💠 Example

```bash
curl -X POST http://localhost:8000/6 \
  -H 'Content-Type: text/plain' \
  -d 'there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.'

{"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}
```
