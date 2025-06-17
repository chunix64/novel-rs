> ⚠️ **Alpha Warning**  
> This project is in **early development (alpha)** and considered a **demo/pre-release**.  
> Features may break or change anytime. Documentation is **incomplete** — expect to explore the source.  
> Feedback, issues, and contributions are highly welcome!

<h1 align="center">Novel-rs</h1>
<p align="center"><i>Realtime multipage novel scraper</i></p>

---

## ✅ Current Site Support

| Site               | Status |
|--------------------|--------|
| docln.net          | ✅     |
| light-novelpub.com | ❌     |

> Currently, only single-page scraping is supported (docln). Multipage and backend service support are planned.

---

## 📥 Installation

### From Release
Download from [Releases](https://github.com/chunix64/novel-rs/releases).

### From Source

```bash
git clone https://github.com/chunix64/novel-rs/
cd novel-rs
cargo install --path ./   # or use cargo build --release
```

> Requires [Cargo](https://doc.rust-lang.org/cargo) and [Git](https://git-scm.com/) (optional — you can extract manually).

Binary output: `./target/release/novel-rs`

---

## 🖥️ Usage

List all options:
```bash
novel-rs --help
```

Sync all novels list:
```bash
novel-rs --sync-items
```

Sync all novel chapters/content:
```bash
novel-rs --sync-contents
```

---

## ⛁ Output Structure

- All output saved to: `/data/`
  - Cache: `/data/cache/`
  - Database: `/data/db/sites/docln.sqlite3`
- Other folders are placeholders for future updates.

---

## 🛠 For Developers

We plan to **separate** the project into:
- `novel-rs-core`
- `novel-rs-cli`

Please help modularize and improve!
