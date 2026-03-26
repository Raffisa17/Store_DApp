#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data product (barang toko)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    id: u64,
    name: String,
    price: u64,
    stock: u32,
}

// Storage key
const PRODUCT_DATA: Symbol = symbol_short!("PRODUCT");

#[contract]
pub struct StoreContract;

#[contractimpl]
impl StoreContract {
    // Ambil semua produk
    pub fn get_products(env: Env) -> Vec<Product> {
        env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env))
    }

    // Tambah produk baru
    pub fn create_product(env: Env, name: String, price: u64, stock: u32) -> String {
        let mut products: Vec<Product> = env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        let product = Product {
            id: env.prng().gen::<u64>(),
            name: name,
            price: price,
            stock: stock,
        };

        products.push_back(product);
        env.storage().instance().set(&PRODUCT_DATA, &products);

        String::from_str(&env, "Produk berhasil ditambahkan")
    }

    // Hapus produk berdasarkan id
    pub fn delete_product(env: Env, id: u64) -> String {
        let mut products: Vec<Product> = env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..products.len() {
            if products.get(i).unwrap().id == id {
                products.remove(i);
                env.storage().instance().set(&PRODUCT_DATA, &products);
                return String::from_str(&env, "Produk berhasil dihapus");
            }
        }

        String::from_str(&env, "Produk tidak ditemukan")
    }
}

mod test;













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/