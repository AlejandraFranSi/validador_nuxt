// Import the libraries and functions we'll use
use chardetng::{EncodingDetector,Iso2022JpDetection, Utf8Detection};
//use polars::prelude::*;
use std::collections::{BTreeMap, BTreeSet,};
use std::{ fs::{File}};
use std::io::{Read, BufReader, BufRead};
use std::sync::Mutex;
use serde::{Serialize};

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
         .manage(ContenedorDatos { 
            filas_completas: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![leer_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct ContenedorDatos {
    pub filas_completas: Mutex<Vec<Vec<String>>>,
}

#[derive(Serialize, Debug)]
pub struct CaracterCorrupto {
    pub caracter: String,
    pub filas: Vec<u64>
}
#[derive(Serialize, Debug)]
pub struct EsquemaColumna {
    pub nombre: String,
    pub tipo: String,
}

#[derive(Serialize)]
pub struct ReporteCsv {
    pub encoding_original: String,
    pub requiere_conversion: bool,
    pub caracteres_corruptos: Vec<CaracterCorrupto>,
    pub total_rows: usize,
    //pub columnas: Vec<String>,
    //pub esquema_columnas: Vec<EsquemaColumna>,
}

fn es_caracter_corrupto(c: char) -> bool {
    let code = c as u32;

    // 1. Validar el rombo de reemplazo directamente
    if c == '\u{FFFD}' {
        return true;
    }
    // 2. Control chars (excluyendo tab, LF, CR)
    if code < 32 && code != 9 && code != 10 && code != 13 {
        return true;
    }
    // 3. Delete char
    if code == 127 {
        return true;
    }

    // 4. Expresiones regulares NORMAL y TYPICAL para español/datos comunes
    // Caracteres NORMALES: a-z, A-Z, 0-9, espacios y puntuación básica
    let es_normal = c.is_ascii_alphanumeric() || c.is_ascii_whitespace() || 
                    ".,;:()\"'¿?¡!-_/".contains(c);

    if !es_normal {
        // Si no es normal, revisamos si al menos es de los TÍPICOS aceptados en español (acentos, eñes, símbolos de pesos, etc.)
        let es_tipico = "áéíóúÁÉÍÓÚñÑüÜ“”\"%°ºª€$".contains(c);
        if es_tipico {
            return false; // Es un acento o eñe perfectamente válido
        } else {
            return true; // Es un "badChar" real (un Mojibake o símbolo extraño)
        }
    }

    false
}

#[tauri::command]
fn leer_csv(ruta_front: String){
    // Confirmar que existe la ruta
    let ruta = ruta_front;
    
    // Vamos a leer el archivo y guardarlos en un vector de bytes
    let file = File::open(&ruta).map_err(|e| e.to_string()).unwrap();
    
    // Vamos a leer solo una parte del archivo para poder identificar el encoding
    let mut partial_reader = BufReader::new(file);
    let mut partial_bytes = vec![0; 4096];
    partial_reader.read(&mut partial_bytes).map_err(|e| e.to_string());

    // Y también vamos a leer todo para poder iterar sobre el contenido
    let file_completo = File::open(&ruta).map_err(|e| e.to_string()).unwrap();
    let mut file_as_bytes: BufReader<File> = BufReader::new(file_completo);

    // Queremos revisar si el encoding es utf8
    let tuviera_errores = encoding_rs::UTF_8.decode(&partial_bytes).2;
    let mut encoding_aplicado = if tuviera_errores {"".to_string()} else { "UTF-8".to_string()} ;
    let mut mapa_caracteres: BTreeMap<char, BTreeSet<u64>> = BTreeMap::new();
    let mut total_filas :usize = 0; 
    // Si el texto no es utf8, vamos a identificar qué encoding se usó
    // Parsearlo como utf8 y almacenarlo en algún lugar
    if tuviera_errores{
        // Constuirmos un detector de encoding en el cual se rechaza la posibilidad de que el resultado sea ISO-2022-JP
        // Adivinamos el encoding, permitiendo que la respuesta sea utf8
        // Decodificamos el texto usando el nuevo encoding. 
        // Si encontramos un caracter corrupto lo agregamos a mapa_caracteres
        // Se revisa si el elemento ya está en mapa_caracteres, si no está se agrega un valor default con or_default()
        // Y luego se llena ese valor default con el valor de fila_actual
        let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
        detector.feed(&partial_bytes, true);
        let encoder = detector.guess(None, Utf8Detection::Allow);
        encoding_aplicado = encoder.name().to_string();
        for (indice, linea) in file_as_bytes.split(b'\n').enumerate() {
            total_filas += 1;
            let line = linea.map_err(|e| e.to_string()).unwrap();
            let encoded_line = encoder.decode(&line).0.to_string();
            let fila_actual = (indice + 1) as u64;
            for c in encoded_line.chars() {
                if es_caracter_corrupto(c) {
                    mapa_caracteres.entry(c).or_default().insert(fila_actual);
                }
            }     
        }
    } else {
         
        for (indice, linea) in file_as_bytes.lines().enumerate() {
        total_filas += 1;
        let line = linea.map_err(|e| e.to_string()).unwrap();
        let fila_actual = (indice + 1) as u64;
        for c in line.chars() {
                if es_caracter_corrupto(c) {
                    mapa_caracteres.entry(c).or_default().insert(fila_actual);
                }
            }
        }
    }
    
    let caracteres_corruptos: Vec<CaracterCorrupto> = mapa_caracteres.into_iter().map(|(caracter,filas)| CaracterCorrupto {
        caracter: caracter.to_string(),
        filas: filas.into_iter().collect()
    }).collect();

    println!("El mapa de caracteres corrputos: {:?}", caracteres_corruptos);  

    //Ok(ReporteCsv{encoding_original, requiere_conversion, caracteres_corruptos, total_rows})

}
