# 🎙️ Si Asisten Tak Terlihat (Invisible Meeting Assistant)

Status: Perencanaan / Brainstorming
Target Platform: macOS (Minimal macOS 13 Ventura)
Tech Stack: Tauri (Rust Backend + React/Tailwind Frontend)

📌 Ringkasan Konsep

Aplikasi native macOS super ringan yang hidup di Menu Bar. Berfungsi untuk merekam audio meeting (Zoom, Teams, Meet) secara diam-diam (seamless), mentranskripsinya secara lokal menggunakan AI untuk menjaga privasi, dan menghasilkan full transcript serta summary berformat Markdown yang siap diintegrasikan dengan Obsidian/Notion. Menggunakan model "Bring Your Own Key" (BYOK) untuk kebebasan memilih LLM.

🏗️ Arsitektur & Teknologi

1. Frontend (Antarmuka Pengguna)

Framework: React (via Vite) + Tailwind CSS + komponen minimalis (mis. shadcn/ui).

Format: Hidup sebagai Menu Bar/System Tray app. Tidak memiliki jendela utama (main window) besar.

Fitur UI:

Tombol Start/Stop Recording.

Indikator status (Merekam / Mentranskripsi / Menghasilkan Summary).

Halaman Settings untuk: Model Whisper, API Key LLM (OpenRouter/Claude), dan direktori Auto-Save Markdown.

2. Backend (Logic & Native API)

Framework: Rust (menggunakan Tauri Commands & Events).

Tugas Utama:

Manajemen Thread agar proses berat tidak mengunci UI.

Komunikasi async antara backend audio/AI dan frontend.

3. Audio Capture (Tantangan Utama macOS)

Teknologi: API resmi Apple ScreenCaptureKit (via crate screencapturekit di Rust).

Metode: Mem-bypass kebutuhan virtual driver (seperti BlackHole).

Isolasi: Menggunakan filter spesifik SCK untuk hanya menangkap audio dari aplikasi target (mis. Zoom) dan mengabaikan aplikasi lain (mis. Spotify). Video diabaikan untuk menghemat CPU.

Konversi: Ekstraksi data CMSampleBuffer Apple dan resampling paksa menjadi format 16kHz, Mono, 16-bit Integer (Syarat mutlak Whisper.cpp).

4. Mesin Transkripsi Lokal (AI Core)

Model: Whisper.cpp (via C/C++ FFI di Rust).

Ukuran Model: Pilihan ukuran small atau medium untuk keseimbangan kecepatan dan akurasi (mampu mendeteksi dan mentranskripsi percampuran Bahasa Indonesia dan Inggris otomatis tanpa training tambahan).

Proses: Mengonsumsi stream audio 16kHz dari backend dan menghasilkan Full Transcript secara lokal (privasi terjamin).

5. Post-Processing & Output

Integrasi LLM (BYOK): Transkripsi lokal yang sudah selesai dikirim ke API pilihan pengguna (OpenRouter, Anthropic, OpenAI) beserta custom prompt untuk perbaikan ejaan dan pembuatan summary/action items.

Format Output Akhir: File Markdown (.md).

Mendukung Frontmatter (YAML) berisi metadata (Tanggal, Durasi, Aplikasi).

Auto-Save langsung ke Vault Obsidian (atau direktori pilihan pengguna) tanpa perlu copy-paste.

🛣️ Peta Jalan Eksekusi (Roadmap)

Fase 1: Eksplorasi Infrastruktur (Proof of Concept)

[ ] Setup environment Rust dan Xcode.

[ ] Buat script Rust kecil menggunakan crate screencapturekit.

[ ] Verifikasi kemampuan memfilter dan menangkap audio spesifik dari Zoom/Meet tanpa menangkap suara sistem lainnya.

[ ] Verifikasi ekstraksi raw audio menjadi format PCM.

Fase 2: Integrasi Mesin Transkripsi Lokal

[ ] Compile Whisper.cpp dan unduh model (small/medium).

[ ] Hubungkan raw audio 16kHz dari SCK (Fase 1) ke mesin Whisper.cpp.

[ ] Tes kemampuan transkripsi code-switching (Indonesia-Inggris).

Fase 3: Struktur Tauri & Komunikasi

[ ] Setup proyek Tauri (React + Rust).

[ ] Buat sistem komunikasi Commands (Frontend panggil Rust) dan Events (Rust kirim progress ke Frontend).

[ ] Pindahkan logika Fase 1 & 2 ke dalam struktur backend Tauri (gunakan manajemen thread/async yang benar).

Fase 4: Integrasi LLM & Output

[ ] Buat halaman Settings di frontend untuk input API Key dan direktori penyimpanan.

[ ] Implementasikan panggilan API (via reqwest di Rust) ke OpenRouter/Claude.

[ ] Logika format Markdown dan Auto-Save (File System writing).

Fase 5: UI & Polish

[ ] Rapikan antarmuka Menu Bar.

[ ] Uji coba skenario end-to-end (mulai meeting, rekam, transkripsi, summary, otomatis masuk Obsidian).

[ ] Handling berbagai error (izin OS ditolak, API error, aplikasi target ditutup).
