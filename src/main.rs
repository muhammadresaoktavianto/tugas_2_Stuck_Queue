// Import modul-modul yang diperlukan dari pustaka standar Rust
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::Write;

// Definisi struktur data Wisata untuk merepresentasikan informasi sebuah objek wisata
#[derive(Debug, Clone)]
struct Wisata {
    nama: String,
    keterangan: String,
    jam_operasional: (String, String),
    hari_operasional: HashSet<String>,
    tarif_tiket: f64,
    lokasi: String,
}

// Fungsi utama program
fn main() {
    // Inisialisasi HashMap data_wisata untuk menyimpan data wisata berdasarkan ID
    let mut data_wisata: HashMap<usize, Wisata> = HashMap::new();
    // Inisialisasi counter untuk memberikan ID unik pada setiap data wisata yang ditambahkan
    let mut counter = 1;

    // Inisialisasi struktur data untuk menyimpan history operasi "Undo"
    let mut undo_stack: Vec<HashMap<usize, Wisata>> = Vec::new();
    // Inisialisasi struktur data untuk menyimpan history operasi "Redo"
    let mut redo_queue: VecDeque<HashMap<usize, Wisata>> = VecDeque::new();

    // Inisialisasi struktur data untuk menyimpan history operasi "Undo" secara keseluruhan
    let mut undo_history: VecDeque<HashMap<usize, Wisata>> = VecDeque::new();
    // Inisialisasi struktur data untuk menyimpan history operasi "Redo" secara keseluruhan
    let mut redo_history: VecDeque<HashMap<usize, Wisata>> = VecDeque::new();

    // Loop utama program
    loop {
        // Menampilkan menu aplikasi
        println!("===== Aplikasi Pariwisata =====");
        println!("1. Tambah Data");
        println!("2. Lihat Data");
        println!("3. Edit Data");
        println!("4. Hapus Data");
        println!("5. Stack");
        println!("6. Queue");
        println!("7. Lihat Stack History");
        println!("8. Lihat Queue History");
        println!("9. Keluar");

        // Membaca pilihan input dari pengguna
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memproses pilihan input pengguna
        match input.trim().parse() {
            Ok(choice) => match choice {
                // Jika pilihan adalah 1, panggil fungsi tambah_data
                1 => {
                    tambah_data(
                        &mut data_wisata,
                        &mut counter,
                        &mut undo_stack,
                        &mut redo_queue,
                        &mut undo_history,
                    );
                }
                // Jika pilihan adalah 2, panggil fungsi lihat_data
                2 => lihat_data(&data_wisata),
                // Jika pilihan adalah 3, panggil fungsi edit_data
                3 => {
                    edit_data(
                        &mut data_wisata,
                        &mut undo_stack,
                        &mut redo_queue,
                        &mut undo_history,
                    );
                }
                // Jika pilihan adalah 4, panggil fungsi hapus_data
                4 => {
                    hapus_data(
                        &mut data_wisata,
                        &mut undo_stack,
                        &mut redo_queue,
                        &mut undo_history,
                    );
                }
                // Jika pilihan adalah 5, panggil fungsi stack
                5 => {
                    stack(
                        &mut data_wisata,
                        &mut undo_stack,
                        &mut redo_queue,
                        &mut undo_history,
                        &mut redo_history,
                    );
                }
                // Jika pilihan adalah 6, panggil fungsi queue
                6 => {
                    queue(
                        &mut data_wisata,
                        &mut undo_stack,
                        &mut redo_queue,
                        &mut undo_history,
                        &mut redo_history,
                    );
                }
                // Jika pilihan adalah 7, panggil fungsi lihat_history untuk "Undo"
                7 => {
                    lihat_history("Undo", &undo_history);
                }
                // Jika pilihan adalah 8, panggil fungsi lihat_history untuk "Redo"
                8 => {
                    lihat_history("Redo", &redo_history);
                }
                // Jika pilihan adalah 9, keluar dari loop utama
                9 => {
                    println!("Keluar dari aplikasi.");
                    break;
                }
                // Jika pilihan tidak sesuai dengan opsi yang ada, tampilkan pesan kesalahan
                _ => println!("Pilihan tidak valid."),
            },
            // Jika pengguna memasukkan input yang bukan angka, tampilkan pesan kesalahan
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi untuk menambahkan data wisata ke dalam HashMap
fn tambah_data(
    data_wisata: &mut HashMap<usize, Wisata>,
    counter: &mut usize,
    undo_stack: &mut Vec<HashMap<usize, Wisata>>,
    redo_queue: &mut VecDeque<HashMap<usize, Wisata>>,
    undo_history: &mut VecDeque<HashMap<usize, Wisata>>,
) {
    // Menampilkan prompt untuk memasukkan data baru
    println!("===== Tambah Data =====");

    // Meminta input pengguna untuk setiap atribut data wisata
    let nama = input("Masukkan nama tempat wisata:");
    let keterangan = input("Masukkan keterangan tempat wisata:");
    let jam_buka = input("Masukkan jam buka tempat wisata (format: HH:mm):");
    let jam_tutup = input("Masukkan jam tutup tempat wisata (format: HH:mm):");
    let hari_operasional = input("Masukkan hari operasional tempat wisata (pisahkan dengan tanda - , contoh: Senin-Selasa):")
        .split('-')
        .map(|s| s.trim().to_string())
        .collect();
    let tarif_tiket = input_f64("Masukkan tarif tiket tempat wisata:");
    let lokasi = input("Masukkan lokasi tempat wisata:");

    // Membuat struktur data Wisata baru
    let wisata_baru = Wisata {
        nama,
        keterangan,
        jam_operasional: (jam_buka, jam_tutup),
        hari_operasional,
        tarif_tiket,
        lokasi,
    };

    // Memasukkan data wisata baru ke dalam HashMap
    data_wisata.insert(*counter, wisata_baru);
    *counter += 1; // Menambah counter untuk ID data selanjutnya

    // Menampilkan pesan bahwa data berhasil ditambahkan
    println!("Data berhasil ditambahkan.\n");

    // Menyimpan salinan HashMap untuk operasi Undo
    undo_stack.push(data_wisata.clone());
    undo_history.push_back(data_wisata.clone());
    redo_queue.clear(); // Menghapus redo_queue setiap kali operasi baru dilakukan
}

// Fungsi untuk mengedit data wisata berdasarkan ID
fn edit_data(
    data_wisata: &mut HashMap<usize, Wisata>,
    undo_stack: &mut Vec<HashMap<usize, Wisata>>,
    redo_queue: &mut VecDeque<HashMap<usize, Wisata>>,
    undo_history: &mut VecDeque<HashMap<usize, Wisata>>,
) {
    // Menampilkan prompt untuk mengedit data
    println!("===== Edit Data =====");

    // Memeriksa apakah data wisata kosong
    if data_wisata.is_empty() {
        println!("Tidak ada data untuk diedit.");
    } else {
        // Meminta input ID data yang ingin diedit
        println!("Masukkan ID data yang ingin diedit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memproses input ID
        match input.trim().parse() {
            Ok(id) => {
                // Jika ID ditemukan dalam data wisata
                if let Some(wisata) = data_wisata.get_mut(&id) {
                    // Meminta input pengguna untuk setiap atribut data yang akan diubah
                    let mut nama = String::new();
                    io::stdin().read_line(&mut nama).expect("Gagal membaca input");

                    let mut keterangan = String::new();
                    io::stdin().read_line(&mut keterangan).expect("Gagal membaca input");

                    let mut lokasi = String::new();
                    io::stdin().read_line(&mut lokasi).expect("Gagal membaca input");

                    let mut tarif_tiket = String::new();
                    io::stdin().read_line(&mut tarif_tiket).expect("Gagal membaca input");
                    let tarif_tiket: f64 = tarif_tiket.trim().parse().unwrap_or(0.0);

                    let mut hari_operasional = String::new();
                    io::stdin().read_line(&mut hari_operasional).expect("Gagal membaca input");
                    let hari_operasional: HashSet<String> =
                        hari_operasional.trim().split('-').map(|s| s.trim().to_string()).collect();

                    let mut jam_buka = String::new();
                    io::stdin().read_line(&mut jam_buka).expect("Gagal membaca input");

                    let mut jam_tutup = String::new();
                    io::stdin().read_line(&mut jam_tutup).expect("Gagal membaca input");

                    // Mengupdate data wisata
                    wisata.nama = nama.trim().to_string();
                    wisata.keterangan = keterangan.trim().to_string();
                    wisata.lokasi = lokasi.trim().to_string();
                    wisata.tarif_tiket = tarif_tiket;
                    wisata.hari_operasional = hari_operasional;
                    wisata.jam_operasional = (jam_buka.trim().to_string(), jam_tutup.trim().to_string());

                    // Menampilkan pesan bahwa data berhasil diubah
                    println!("Data berhasil diubah.\n");

                    // Menyimpan salinan HashMap untuk operasi Undo
                    undo_stack.push(data_wisata.clone());
                    undo_history.push_back(data_wisata.clone());
                    redo_queue.clear(); // Menghapus redo_queue setiap kali operasi baru dilakukan
                } else {
                    println!("ID tidak ditemukan.");
                }
            }
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi untuk menghapus data wisata berdasarkan ID
fn hapus_data(
    data_wisata: &mut HashMap<usize, Wisata>,
    undo_stack: &mut Vec<HashMap<usize, Wisata>>,
    redo_queue: &mut VecDeque<HashMap<usize, Wisata>>,
    undo_history: &mut VecDeque<HashMap<usize, Wisata>>,
) {
    // Menampilkan prompt untuk menghapus data
    println!("===== Hapus Data =====");

    // Memeriksa apakah data wisata kosong
    if data_wisata.is_empty() {
        println!("Tidak ada data untuk dihapus.");
    } else {
        // Meminta input ID data yang ingin dihapus
        println!("Masukkan ID data yang ingin dihapus:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        // Memproses input ID
        match input.trim().parse() {
            Ok(id) => {
                // Jika ID ditemukan dalam data wisata
                if let Some(_) = data_wisata.remove(&id) {
                    // Menampilkan pesan bahwa data berhasil dihapus
                    println!("Data berhasil dihapus.\n");

                    // Menyimpan salinan HashMap untuk operasi Undo
                    undo_stack.push(data_wisata.clone());
                    undo_history.push_back(data_wisata.clone());
                    redo_queue.clear(); // Menghapus redo_queue setiap kali operasi baru dilakukan
                } else {
                    println!("ID tidak ditemukan.");
                }
            }
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

// Fungsi untuk mengembalikan state data wisata ke versi sebelumnya menggunakan konsep "Stack"
fn stack(
    data_wisata: &mut HashMap<usize, Wisata>,
    undo_stack: &mut Vec<HashMap<usize, Wisata>>,
    redo_queue: &mut VecDeque<HashMap<usize, Wisata>>,
    undo_history: &mut VecDeque<HashMap<usize, Wisata>>,
    redo_history: &mut VecDeque<HashMap<usize, Wisata>>,
) {
    // Memeriksa apakah terdapat state sebelumnya dalam undo_stack
    if undo_stack.len() > 1 {
        // Pop state saat ini dari undo_stack
        redo_queue.push_front(undo_stack.pop().unwrap());
        // Mengembalikan state sebelumnya
        *data_wisata = undo_stack.last().unwrap().clone();
        // Menampilkan pesan bahwa operasi "Undo" (Stack) berhasil
        println!("Undo (Stack) berhasil.\n");

        // Menyimpan salinan HashMap untuk operasi "Redo"
        redo_history.push_front(data_wisata.clone());
        undo_history.pop_back(); // Menghapus operasi terakhir dari undo_history
    } else {
        // Menampilkan pesan bahwa operasi "Undo" (Stack) tidak dapat dilakukan
        println!("Undo (Stack) tidak dapat dilakukan.\n");
    }
}

// Fungsi untuk mengembalikan state data wisata ke versi sebelumnya menggunakan konsep "Queue"
fn queue(
    data_wisata: &mut HashMap<usize, Wisata>,
    undo_stack: &mut Vec<HashMap<usize, Wisata>>,
    redo_queue: &mut VecDeque<HashMap<usize, Wisata>>,
    undo_history: &mut VecDeque<HashMap<usize, Wisata>>,
    redo_history: &mut VecDeque<HashMap<usize, Wisata>>,
) {
    // Memeriksa apakah terdapat state yang

    if let Some(new_state) = redo_queue.pop_front() {
        // Save current state for potential undo
        undo_stack.push(data_wisata.clone());
        // Apply redo state
        *data_wisata = new_state;
        println!("Redo (Queue) berhasil.\n");

        // Simpan salinan HashMap untuk operasi Undo
        undo_history.push_back(data_wisata.clone());
        redo_history.pop_front(); // Hapus operasi terakhir dari redo_history
    } else {
        println!("Redo (Queue) tidak dapat dilakukan.\n");
    }
}

fn lihat_history(history_type: &str, history: &VecDeque<HashMap<usize, Wisata>>) {
    println!("===== Lihat {} History =====", history_type);

    for (index, state) in history.iter().enumerate() {
        println!("State {}: {:#?}", index + 1, state); // Menggunakan {:#?} agar tampilan lebih rapi
    }

    println!();
}

fn lihat_data(data_wisata: &HashMap<usize, Wisata>) {
    println!("===== Lihat Data =====");

    if data_wisata.is_empty() {
        println!("Tidak ada data.");
    } else {
        let max_lokasi_length = data_wisata
            .values()
            .map(|wisata| wisata.lokasi.len())
            .max()
            .unwrap_or(10);

        println!(
            "{:<6} | {:<20} | {:<20} | {:<width$} | {:<15} | {:<20} | {:<20}",
            "ID",
            "Nama",
            "Keterangan",
            "Lokasi",
            "Tarif Tiket",
            "Hari Operasional",
            "Jam Operasional",
            width = max_lokasi_length
        );

        println!(
            "---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------"
        );

        for (id, wisata) in data_wisata {
            println!(
                "{:<6} | {:<20} | {:<20} | {:<width$} | {:<15} | {:<20} | {:<20}",
                id,
                &wisata.nama,
                &wisata.keterangan,
                &wisata.lokasi,
                                &wisata.tarif_tiket,
                &wisata.hari_operasional
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", "),
                format!("{} - {}", &wisata.jam_operasional.0, &wisata.jam_operasional.1),
                width = max_lokasi_length
            );
        }

        println!();
    }
}

fn input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    input.trim().to_string()
}

fn input_f64(prompt: &str) -> f64 {
    loop {
        match input(prompt).parse() {
            Ok(value) => return value,
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}