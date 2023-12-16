use std::collections::HashMap;
use std::io;

struct Barang {
    kode: String,
    nama: String,
    stok: i32,
}

impl Barang {
    fn new(kode: &str, nama: &str, stok: i32) -> Barang {
        Barang {
            kode: kode.to_string(),
            nama: nama.to_string(),
            stok,
        }
    }
}

fn main() {
    let mut daftar_barang: HashMap<String, Barang> = HashMap::new();

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah barang");
        println!("2. Lihat daftar barang");
        println!("3. Edit daftar barang");
        println!("4. Hapus barang");
        println!("5. Keluar");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca baris");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukan tidak valid. Masukkan nomor yang valid.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Masukkan kode barang:");
                let mut kode_barang = String::new();
                io::stdin().read_line(&mut kode_barang).expect("Gagal membaca baris");
                let kode_barang = kode_barang.trim().to_string();

                println!("Masukkan nama barang:");
                let mut nama_barang = String::new();
                io::stdin().read_line(&mut nama_barang).expect("Gagal membaca baris");
                let nama_barang = nama_barang.trim().to_string();

                println!("Masukkan stok barang:");
                let mut stok_barang = String::new();
                io::stdin().read_line(&mut stok_barang).expect("Gagal membaca baris");
                let stok_barang: i32 = match stok_barang.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Stok tidak valid. Masukkan angka yang valid.");
                        continue;
                    }
                };

                let barang = Barang::new(&kode_barang, &nama_barang, stok_barang);
                daftar_barang.insert(kode_barang.clone(), barang);

                println!("Barang {} berhasil ditambahkan.", nama_barang);
            }
            2 => {
                println!("Daftar Barang:");
                for barang in daftar_barang.values() {
                    println!(
                        "Kode: {}, Nama: {}, Stok: {}",
                        barang.kode, barang.nama, barang.stok
                    );
                }
            }
            3 => {
                println!("Masukkan kode barang yang ingin diubah:");
                let mut kode_barang = String::new();
                io::stdin().read_line(&mut kode_barang).expect("Gagal membaca baris");
                let kode_barang = kode_barang.trim().to_string();

                if let Some(barang) = daftar_barang.get_mut(&kode_barang) {
                    println!("Masukkan nama barang baru:");
                    let mut new_nama_barang = String::new();
                    io::stdin().read_line(&mut new_nama_barang).expect("Gagal membaca baris");
                    barang.nama = new_nama_barang.trim().to_string();

                    println!("Masukkan stok barang baru:");
                    let mut new_stok_barang = String::new();
                    io::stdin().read_line(&mut new_stok_barang).expect("Gagal membaca baris");
                    barang.stok = match new_stok_barang.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Stok tidak valid. Masukkan angka yang valid.");
                            continue;
                        }
                    };

                    println!("Barang {} berhasil diubah.", kode_barang);
                } else {
                    println!("Barang {} tidak ditemukan.", kode_barang);
                }
            }
            4 => {
                println!("Masukkan kode barang yang ingin dihapus:");
                let mut kode_barang = String::new();
                io::stdin().read_line(&mut kode_barang).expect("Gagal membaca baris");
                let kode_barang = kode_barang.trim().to_string();

                if let Some(_) = daftar_barang.remove(&kode_barang) {
                    println!("Barang {} berhasil dihapus.", kode_barang);
                } else {
                    println!("Barang {} tidak ditemukan.", kode_barang);
                }
            }
            5 => {
                println!("Terima kasih!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Masukkan nomor yang valid.");
            }
        }
    }
}
