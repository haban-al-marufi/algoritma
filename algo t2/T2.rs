use std::{vec, collections::{VecDeque}};

#[derive(Debug, Clone)]
struct Barang {
    kode: String,
    nama: String,
    stok: usize,
}

struct InventoryManager {
    stack: Vec<Barang>,
    queue: VecDeque<Barang>,
}

impl InventoryManager {
    fn new() -> Self {
        InventoryManager {
            stack: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    fn tambah_barang(&mut self, kode: String, nama: String, stok: usize) {
        let barang = Barang { kode, nama, stok };
        self.stack.push(barang.clone());
        self.queue.push_back(barang);
    }

    fn lihat_daftar_barang(&self) {
        println!("Daftar Barang:");

        // Menampilkan data dari stack
        println!("Dari Stack :");
        for barang in self.stack.iter().rev() {
            println!("{:?}", barang);
        }

        // Menampilkan data dari queue
        println!("Dari Queue :");
        for barang in &self.queue {
            println!("{:?}", barang);
        }
    }

    fn edit_daftar_barang(&mut self, kode: &str, new_nama: String, new_stok: usize) {
        // Cari data dengan kode yang sesuai, lalu ubah nama dan stok
        for barang in &mut self.stack {
            if barang.kode == kode {
                barang.nama = new_nama.clone();
                barang.stok = new_stok;
                break;
            }
        }

        for barang in &mut self.queue {
            if barang.kode == kode {
                barang.nama = new_nama.clone();
                barang.stok = new_stok;
                break;
            }
        }
    }

    fn hapus_barang(&mut self, kode: &str) {
        // Hapus data dengan kode yang sesuai dari stack dan queue
        self.stack.retain(|barang| barang.kode != kode);
        self.queue.retain(|barang| barang.kode != kode);
    }
}

fn main() {
    let mut inventory_manager = InventoryManager::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Barang");
        println!("2. Lihat Daftar Barang");
        println!("3. Edit Daftar Barang");
        println!("4. Hapus Barang");
        println!("5. Keluar");

        let choice: u32 = read_line().trim().parse().expect("Invalid input");

        match choice {
            1 => {
                // Tambah Barang
                println!("Masukkan Kode Barang:");
                let kode = read_line().trim().to_string();

                println!("Masukkan Nama Barang:");
                let nama = read_line().trim().to_string();

                println!("Masukkan Stok:");
                let stok: usize = read_line().trim().parse().expect("Invalid Stok");

                inventory_manager.tambah_barang(kode, nama, stok);
            }
            2 => {
                // Lihat Daftar Barang
                inventory_manager.lihat_daftar_barang();
            }
            3 => {
                // Edit Daftar Barang
                println!("Masukkan Kode Barang yang akan diubah:");
                let kode = read_line().trim().to_string();

                println!("Masukkan Nama Barang Baru:");
                let new_nama = read_line().trim().to_string();

                println!("Masukkan Stok Baru:");
                let new_stok: usize = read_line().trim().parse().expect("Invalid Stok");

                inventory_manager.edit_daftar_barang(&kode, new_nama, new_stok);
            }
            4 => {
                // Hapus Barang
                println!("Masukkan Kode Barang yang akan dihapus:");
                let kode = read_line().trim().to_string();

                inventory_manager.hapus_barang(&kode);
            }
            5 => {
                // Keluar
                println!("Program Berakhir.");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Silakan coba lagi.");
            }
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
