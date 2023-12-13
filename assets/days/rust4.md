# 🎄 Day 4: What do you call a serialized reindeer? Serdeer!

*Under the soft glow of the Northern Lights, Santa's reindeer are training for the big night. But, oh deer! The reindeer's stats have been serialized into an unknown format after a playful elf accidentally spilled hot cocoa on the central computer. The data needs to be deserialized so the reindeer team can be assembled based on their combined strength.*

## ⭐ Task 1: Reindeer cheer

The task is to create a POST endpoint `/4/strength` that calculates the combined strength of a group of reindeer, so that Santa knows if they can pull his sled through the skies.

The input to the endpoint is a JSON array containing information about each reindeer. Each reindeer is represented as an object with two attributes: `"name"` (string) and `"strength"` (integer). Collect the strength of each reindeer and respond with the sum.

### 🔔 Tips 

- [serde](https://docs.rs/serde/latest/serde/)
- [serde_json](https://docs.rs/serde_json/latest/serde_json/)
- [JSON in Axum](https://docs.rs/axum/latest/axum/struct.Json.html)
- [JSON in Actix Web](https://actix.rs/docs/request)
- [JSON in Rocket](https://rocket.rs/v0.5/guide/requests/#json)

### 💠 Example

```bash
curl -X POST http://localhost:8000/4/strength \
  -H 'Content-Type: application/json' \
  -d '[
    { "name": "Dasher", "strength": 5 },
    { "name": "Dancer", "strength": 6 },
    { "name": "Prancer", "strength": 4 },
    { "name": "Vixen", "strength": 7 }
  ]'

22
```

---

## 🎁 Task 2: Cursed candy eating contest (150 bonus points)

This time, there is some more data for each reindeer (see example).
The endpoint is called `/4/contest`, because the reindeer are now  going to compare the attributes of the reindeer and present a summary of the winners.

There is at least one reindeer participating in the contest, and there is never a tie for first place.

For some reason, one of the field names in the input seems to still be a bit corrupted from the incident. Guess we just have to deal with it anyways.

The output should be a JSON object containing a summary of the winners (see example).

### 💠 Example Input

```bash
curl -X POST http://localhost:8000/4/contest \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "name": "Dasher",
      "strength": 5,
      "speed": 50.4,
      "height": 80,
      "antler_width": 36,
      "snow_magic_power": 9001,
      "favorite_food": "hay",
      "cAnD13s_3ATeN-yesT3rdAy": 2
    },
    {
      "name": "Dancer",
      "strength": 6,
      "speed": 48.2,
      "height": 65,
      "antler_width": 37,
      "snow_magic_power": 4004,
      "favorite_food": "grass",
      "cAnD13s_3ATeN-yesT3rdAy": 5
    }
  ]'
```

### 💠 Example Output

*Note: JSON output examples are sometimes formatted. Output from your endpoint does not need to be formatted. The output is parsed and checked as a value.*

```json
{
  "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
  "tallest": "Dasher is standing tall with his 36 cm wide antlers",
  "magician": "Dasher could blast you away with a snow magic power of 9001",
  "consumer": "Dancer ate lots of candies, but also some grass"
}
```
