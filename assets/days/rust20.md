# 🎄 Day 20: Git good

*Santa frowned, his usually merry eyes scanning the data on the screen before him. Something wasn't right. He pulled up the database of gift orders worldwide, but there was a noticeable gap in the records. This was a serious issue, and Santa knew the implications immediately - missing orders meant missing gifts and unhappy children. Just the thought of it made his jolly cheer fade a bit, replaced by a hint of worry.*

*"Time for a trip down memory lane," Santa mumbled to himself as he trudged his way towards the archives. The archives were a labyrinth of shelves, filled to the brim with old records and endless stacks of papers detailing all past Christmases. It was a treasure trove of information that had slowly been digitized, but the older records, the ones that were now in question, still lay tucked in the musty corners of the archive.*

*It wouldn't be an easy task, and even Santa knew it could take hours, maybe even days. But every child mattered, and Santa would not rest until every record was found and every child got their rightful gift. With a deep breath, Santa began his journey in the archives, determined to fill in the gaps and ensure a merry Christmas for all.*

## ⭐ Task 1: Archive Analysis

To find some very old gift order records, Santa needs to dig deep into the archives.
You offer to help him parse and analyze the archive files.

Create a POST endpoint `/20/archive_files` that receives a `tar` archive file in binary format and returns the number of files inside, and another POST endpoint `/20/archive_files_size` that does the same thing but instead returns the sum of all file sizes.

### 🔔 Tips

- [tar](https://crates.io/crates/tar)

### 💠 Example

Download the test file [northpole20231220.tar](https://cch23.shuttleapp.rs/assets/northpole20231220.tar) and use it like this:

```bash
curl -X POST http://localhost:8000/20/archive_files \
  -H 'Content-Type: application/x-tar' \
  --data-binary '@northpole20231220.tar'

6
```

```bash
curl -X POST http://localhost:8000/20/archive_files_size \
  -H 'Content-Type: application/x-tar' \
  --data-binary '@northpole20231220.tar'

1196282
```

---

## 🎁 Task 2: Git Santa his cookie back (350 bonus points)

Santa lost his cookie recently, and can't find it anymore.
Good thing that everything in the north pole is logged in the git logs!
By using them, we can figure out the last one that saw it.

Add the endpoint POST `/20/cookie`.
It will receive a tar archive just like before, but this time it contains a `.git` directory with a repository structure inside.
Extract the archive to somewhere on the file system (for example in a temporary directory with the crate [tempfile](https://crates.io/crates/tempfile)) and find the answer to the following instruction that Santa wrote down:

*Find the name of the commit author and the commit hash of the most recent commit on the branch `christmas` that has a tree that contains a file called `santa.txt` (in any directory) with the string `COOKIE` anywhere inside of it.*

- There are no merge commits in the repo (all commits have one parent, except the root commit).

### 💠 Example

Test file: [cookiejar.tar](https://cch23.shuttleapp.rs/assets/cookiejar.tar)

```bash
curl -X POST http://localhost:8000/20/cookie \
  -H 'Content-Type: application/x-tar' \
  --data-binary '@cookiejar.tar'

Grinch 71dfab551a1958b35b7436c54b7455dcec99a12c
```

---

Authors: [jonaro00](https://github.com/jonaro00), [orhun](https://github.com/orhun)
