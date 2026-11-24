# KiraTuber — Anime Broadcast Studio Design System (`DESIGN.md`)

Dokumen desain ini merumuskan identitas visual **Desktop Anime VTuber Studio** untuk aplikasi desktop **KiraTuber (Tauri v2)**, menggabungkan prinsip kualitas UI dari **Obsidian Vault (`Resource.md`)** serta referensi repositori UI game anime & HUD open-source.

---

## 1. Sumber Referensi & Prinsip Desain

### Dari Obsidian `Resource.md`
1. **Impeccable (`pbakaus/impeccable` — impeccable.style) & Unslop (`heydamir/unslop`)**
   - **Anti-AI-Slop Palette:** Menghindari gradien ungu-biru generik ala landing page SaaS. Semua warna netral diberi *tint* selaras rona tema (*cool ice-slate* untuk mode siang anime, *deep starlight navy* untuk mode idol malam, dan *comic ink charcoal* untuk mode broadcast urban).
   - **Tactile Cel-Shaded Depth:** Menggantikan *blurry glow* berlebihan dengan garis tinta tegas (`1.5px–2px solid`) dan bayangan lipatan *cel-shaded* keras (`0 3px 0 var(--shadow-hard)`), sehingga tombol terasa seperti panel fisik *Stream Deck* / konsol anime.
   - **Tabular Numerals & Hirarki Tegas:** Semua angka slider, persentase *noise gate*, durasi detik, dan derajat rotasi menggunakan `font-variant-numeric: tabular-nums` agar tidak bergeser saat nilainya berubah *real-time*.
   - **Reduced Motion & Aksesibilitas:** Mendukung penuh navigasi keyboard (`:focus-visible`), atribut ARIA, dan `@media (prefers-reduced-motion: reduce)`.
2. **Awesome Claude Design (`VoltAgent/awesome-claude-design`)**
   - Struktur *Design System Tokens* terdokumentasi rapi di `DESIGN.md` agar konsisten saat aplikasi dikembangkan lebih lanjut.
3. **Frontend UI Skill (`ibelick/ui-skills`)**
   - Fokus pada ergonomi **aplikasi desktop native** (bukan website marketing): navigasi tab kategori cepat tanpa *scroll* panjang, kepadatan kontrol tinggi namun lega, dan animasi berbasis *compositor* (`transform`, `opacity`).

### Dari Repositori & Sistem UI Anime / Game Open-Source Tambahan
1. **Schale OS / *Blue Archive* Tactical-Kawaii UI (`DoomVoss/BASpark`, `CialloKing/ba-click-fx`)**
   - Palet cerah khas anime modern (*Ice White*, *Sky Cyan* `#0ea5e9`, *Sakura Coral* `#ff5c8a`), aksen sudut miring (*skewed parallelogram badges* `skewX(-10deg)`), pola titik komik *halftone* halus (`radial-gradient`), dan *hard cel-shadow*.
2. **Augmented UI (`propjockey/augmented-ui`) & Augma AR HUD (`YunYouJun/augma`)**
   - Potongan sudut mekanikal (*chamfered corners* via `clip-path: polygon(...)`) pada kartu aktif, indikator `ON AIR`, dan bingkai editor.
3. **Urban Broadcast HUD (*Zenless Zone Zero* / Cybercore)**
   - Garis diagonal *broadcast hazard stripes*, indikator VU meter bersegmen ala equalizer studio, dan *3D arcade keycaps* untuk *hotkey* ekspresi/gerakan.

---

## 2. Sistem 4 Tema Studio Anime (`data-theme`)

Pengguna dapat memilih 4 preset suasana studio di tab **⚙️ Layar & App** (tersimpan otomatis di `localStorage`):

| Token Peran | `schale-dark` (Anime Gelap — Default) | `schale` (Schale Cerah) | `idol` (Idol Malam) | `zzz` (Cyber ZZZ) |
| :--- | :--- | :--- | :--- | :--- |
| **Karakteristik** | Gelap modern *tactical-anime* dengan Sky Cyan & Sakura | Cerah, bersih, *tactical-kawaii* ala akademi anime | Gelap hangat panggung VTuber malam | Kontras tinggi bergaya komik/mecha urban |
| `--panel` | `#0e1422` | `#edf4fc` | `#101626` | `#121317` |
| `--surface` | `#151f33` | `#ffffff` | `#182138` | `#1b1d24` |
| `--surface-raised` | `#1d2a44` | `#e4effb` | `#212d4c` | `#252832` |
| `--line` (Garis Tinta) | `#2d4066` | `#b9d1ea` | `#31436b` | `#3a3f4d` |
| `--text` | `#eef5ff` | `#152336` | `#f2f6ff` | `#f4f5f8` |
| `--muted` | `#98aecf` | `#4e6683` | `#9eb2d4` | `#a1a8b8` |
| `--accent` (Utama) | `#38bdf8` (Sky Cyan) | `#0ea5e9` (Sky Cyan) | `#ff6b9d` (Sakura Idol) | `#c8ff00` (Acid Lime) |
| `--accent-2` (Sekunder) | `#ff5d8f` (Sakura Coral) | `#ff5688` (Sakura Coral) | `#38bdf8` (Holo Cyan) | `#ff6b2b` (Signal Orange) |
| `--shadow-hard` | `#070b13` | `#c3daef` | `#080c16` | `#07080a` |

---

## 3. Tipografi (Offline-Safe untuk Desktop `.exe`)

- **Display / Judul Bagian & Badge HUD (`--font-display`):**
  `"Chakra Petch", "Bahnschrift", "M PLUS Rounded 1c", "Yu Gothic UI", "Segoe UI Variable Display", sans-serif`
  *(Memberi karakter geometris mecha/anime tegas; pada Windows offline otomatis jatuh ke `Bahnschrift` + `Yu Gothic UI` yang tetap berkarakter tajam).*
- **UI Body (`--font`):**
  `"M PLUS Rounded 1c", "Yu Gothic UI", "Segoe UI Variable Text", "Segoe UI", system-ui, sans-serif`
- **Monospace / Hotkey / Angka Presisi (`--font-mono`):**
  `"JetBrains Mono", "Cascadia Mono", Consolas, monospace`

---

## 4. Aturan Keselamatan OBS & Chroma Key (Wajib)

Karena **KiraTuber** ditangkap oleh streamer di **OBS Studio** (melalui *Window Capture* atau *Game Capture* + *Chroma Key*):
1. **Stage Murni (`#stage`):** Kanvas `#stage` **dilarang** menampilkan dekorasi partikel, teks watermark, atau pola *halftone* di atas warna Chroma Key (`#00b140`, `#00ff00`, `#0000ff`) maupun mode `transparent`.
2. **Mode Sembunyi Panel (`body.hide` / Tombol `H`):** Saat panel disembunyikan untuk *streaming*, seluruh elemen HUD jendela (`#windowDragHandle`, panduan *edit mode*, dan sidebar `#panel`) benar-benar tersembunyi sehingga tangkapan layar OBS 100% bersih.
3. **Panduan Rigging (`body.editing`):** Bingkai sudut pembantu posisi karakter hanya tampil saat pengguna menekan **Atur karakter** dan otomatis hilang ketika selesai diatur atau saat panel disembunyikan.

---

## 5. Anatomi Komponen Desktop Baru

1. **Frameless Studio Header & Live Broadcast Pill:**
   - Menampilkan logo studio bersudut miring, status **SIARAN / SIAGA / MIC MATI** yang bereaksi langsung saat pengguna berbicara, pemilih tema cepat, serta tombol kontrol jendela Tauri (*Minimize*, *Sembunyikan Panel `[H]`*, *Close*).
2. **4-Tab Studio Deck Rail (`Live`, `Aksi`, `Gerak`, `Sistem` + `Semua`):**
   - Mengubah tumpukan formulir panjang menjadi dek kontrol cepat khas aplikasi desktop kreatif.
3. **Kartu Slot Karakter Visual (`#sets`):**
   - Menampilkan *thumbnail* mini karakter secara langsung pada bingkai kotak-kotak transparan, tombol *keycap* 3D (`Q/W/E/R`), status `AKTIF`, dan slot unggah **Diam** & **Bicara** berdampingan.
4. **Dek Ekspresi & Gerakan ala Stream Deck (`#exprs` & `#gest`):**
   - Dilengkapi pratinjau *thumbnail* ekspresi mini, klik cepat untuk menguji ekspresi/gerakan, serta tombol *keycap* mekanikal.
5. **Maskot Chibi Anime Bawaan (`face()`):**
   - 4 varian karakter chibi anime bergaya *cel-shaded* lengkap dengan *headset* telinga kucing/kelinci/bintang dan ekspresi mulut diam/bicara agar aplikasi langsung terlihat hidup sejak pertama kali diunduh.
