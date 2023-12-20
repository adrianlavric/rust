use std::{
    env, fs,
    path::Path,
    process::{self},
    sync::{Arc, Mutex},
    thread,
};

use regex::Regex;

fn main() {
    let parametri: Vec<String> = env::args().skip(1).collect(); // iau doar path-ul si eventualele filtre, sar peste numele executabilului

    if parametri.is_empty() || parametri.iter().any(|parametru| parametru == "--help") {
        println!("Sintaxa: .\\target\\debug\\proiect.exe <cale> [--filter <regex>]");
        process::exit(0);
    }

    let cale = Path::new(&parametri[0]);

    if !cale.is_dir() {
        // verific calea sa fie director

        println!("Cale invalida");
        process::exit(0);
    }

    let mut filtre = Vec::new();

    for (i, parametru) in parametri.iter().enumerate().skip(1) {
        // pun filtrele intr-un vector
        if parametru == "--filter" {
            if let Some(expresie) = parametri.get(i + 1) {
                if let Ok(filtru) = Regex::new(expresie) {
                    filtre.push(filtru);
                } else {
                    println!("Expresie invalida");
                    process::exit(0);
                }
            }
        }
    }

    let dir_size = Arc::new(Mutex::new(0)); // pointer la size, doar un thread il poate modifica simultan

    get_size(cale, filtre, Arc::clone(&dir_size));

    let dir_size = dir_size.lock().unwrap();
    let dir_size_in_gb = *dir_size as f64 / (1073741824.000);
    let dir_size_in_mb = *dir_size as f64 / (1048576.000);

    println!(
        "{:.1}gb, {:.2}mb ({} bytes)",
        dir_size_in_gb, dir_size_in_mb, *dir_size
    );
}

fn get_size(cale: &Path, filtre: Vec<Regex>, dir_size: Arc<Mutex<u64>>) {
    let files = match fs::read_dir(cale) {
        // deschid directorul
        Ok(files) => files,
        Err(_) => process::exit(1),
    };

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for file_deschis in files {
        let file = match file_deschis {
            // le verific pe cele care s-au putut deschide
            Ok(file) => file,
            Err(_) => continue,
        };

        if file.path().is_dir() {
            let clona_dir_size = Arc::clone(&dir_size);
            let clona_filtre = filtre.clone();
            let clona_cale = file.path().clone();

            let handle = thread::spawn(move || {
                // fac un thread nou pentru fiecare director si apelez din nou functia
                get_size(&clona_cale, clona_filtre, clona_dir_size);
            });

            threads.push(handle);
        }

        if file.path().is_file() {
            if !filtre.is_empty() {
                //verific daca fisierul se potriveste cu macar un filtru
                if filtre
                    .iter()
                    .any(|filtru| filtru.is_match(&file.path().to_string_lossy()))
                {
                    let size = match fs::metadata(&file.path()) {
                        Ok(metadata) => metadata.len(),
                        Err(_) => continue,
                    };

                    let mut dir_size = dir_size.lock().unwrap();
                    *dir_size += size;
                }
            } else {
                //daca nu am niciun filtru in linia de comanda adun size-ul tuturor fisierelor
                let size = match fs::metadata(&file.path()) {
                    Ok(metadata) => metadata.len(),
                    Err(_) => continue,
                };

                let mut dir_size = dir_size.lock().unwrap();
                *dir_size += size;
            }
        }
    }

    for thread in threads {
        // astept sa se termine fiecare thread
        thread.join().unwrap();
    }
}
