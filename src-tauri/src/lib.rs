// Import the libraries and functions we'll use
use chardetng::{EncodingDetector,Iso2022JpDetection, Utf8Detection};
use polars::prelude::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet,};
use std::{ fs::{File}};
use std::io::{Read, BufReader, BufRead, Cursor};
use std::sync::Mutex;
use serde::{Serialize};
use tauri::State;
//use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
         .manage(ContenedorDatos { 
            dataframe: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![leer_csv, fetch_rows])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    pub encoding_aplicado: String,
    //pub requiere_conversion: bool,
    pub caracteres_corruptos: Vec<CaracterCorrupto>,
    pub total_filas: usize,
    //pub columnas: Vec<String>,
    pub esquema_columnas: Vec<EsquemaColumna>,
}

pub struct ContenedorDatos {
    pub dataframe: Mutex<Option<DataFrame>>,
    //pub referencia: Mutex<Option<DataFrame>>,
    //pub ruta_original: Mutex<Option<String>>,
    //pub ruta_sugerida: Mutex<Option<String>>
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

/**
 * Esta función se encarga de leer el archivo y crear el dataframe. Para ello ocurren varias cosas:
 * 1. Primero lee únicamente una parte del archivo para identificar el encoding.
 * 2. Si el encoding no es UTF-8, construye un encoder que lo pasa a UTF-8.
 * 3. Leemos el archivo completo, iterando por sus líneas. 
 * 3.1. Si el encoding no es UTF-8, se convierte la línea a UTF-8
 * 3.2. Se identifican los caracteres corruptos de cada línea
 * 3.3. Se agrega cada línea a un vector
 * 4. Se construye un DataFrame a partir del vector
 * 5. Se guarda el Dataframe en el estado global de tauri
 */
#[tauri::command]
fn leer_csv(ruta_front: String, state: State<'_, ContenedorDatos>) -> Result<ReporteCsv, String>{
    let ruta = ruta_front;    
    //let file = File::open("documento.csv").map_err(|_| "No se pudo abrir el archivo solicitado. Confirma que la ruta exista.".to_string())?;
    let file = File::open(&ruta).map_err(|_| "No se pudo abrir el archivo solicitado. Confirma que la ruta exista.".to_string())?;

    let mut partial_reader = BufReader::new(file);
    let mut partial_bytes = vec![0; 4096];
    let _reading = partial_reader.read(&mut partial_bytes).map_err(|_| "No se pudo leer el archivo.".to_string());

    let file_completo = File::open(&ruta).map_err(|_| "No se pudo abrir el archivo. Intenta de nuevo".to_string()).unwrap();
    let file_as_bytes: BufReader<File> = BufReader::new(file_completo);

    let mut contenido_final = Vec::new();
    let tuviera_errores = encoding_rs::UTF_8.decode(&partial_bytes).2;
    let mut encoding_aplicado = if tuviera_errores {"".to_string()} else { "UTF-8".to_string()} ;
    let mut mapa_caracteres: BTreeMap<char, BTreeSet<u64>> = BTreeMap::new();
    if tuviera_errores{
        let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
        detector.feed(&partial_bytes, true);
        let encoder = detector.guess(None, Utf8Detection::Allow);
        encoding_aplicado = encoder.name().to_string();
        for (indice, linea) in file_as_bytes.split(b'\n').enumerate() {
            let line = linea.map_err(|_| "Ocurrió un error al iterar sobre las filas. Intentalo de nuevo.".to_string())?;
            let encoded_line = encoder.decode(&line).0.to_string();
            contenido_final.extend_from_slice(encoded_line.as_bytes());
            contenido_final.extend_from_slice(b"\n");

            let fila_actual = (indice + 1) as u64;
            for c in encoded_line.chars() {
                if es_caracter_corrupto(c) {
                    mapa_caracteres.entry(c).or_default().insert(fila_actual);
                }
            }     
        }
    } else { 
        for (indice, linea) in file_as_bytes.lines().enumerate() {
        let line = linea.map_err(|_|"Ocurrió un error al iterar sobre las filas. Intentalo de nuevo.")?;
        contenido_final.extend_from_slice(line.as_bytes());
        contenido_final.extend_from_slice(b"\n");

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

    let cursor = Cursor::new(contenido_final);
    let mut esquema_columnas: Vec<EsquemaColumna> = Vec::new();  
    let df = CsvReader::new(cursor).with_options(
        CsvReadOptions::default()
            .with_has_header(true)
        ).finish().map_err(|_| "No se pudo construir el DataFrame. Intentalo de nuevo".to_string())?;
    let total_filas = df.height();

    for column in df.columns(){
        let nombre = column.as_materialized_series().name().to_string();
        let tipo = column.as_materialized_series().dtype().to_string();
        esquema_columnas.push(EsquemaColumna{nombre, tipo});
    }

    let mut guardado = state.dataframe.lock().map_err(|_| "Error al bloquear el estado")?;
    *guardado = Some(df);

    Ok(ReporteCsv{encoding_aplicado, caracteres_corruptos, total_filas, esquema_columnas})

}


/**
 * Esta función pide un bloque de tamaño block size a partir del indice start_index
 */
#[tauri::command]
fn fetch_rows(start_index: usize, block_size: usize, state: tauri::State<'_, ContenedorDatos>) -> Result<Value, String> {
    let start_index = if start_index == 1{0}else{(start_index -1)  * block_size };
    let coerced_index: i64 = start_index.try_into().map_err(|_| "No se pudieron recuperar las filas".to_string())?; 
    let rows = state.dataframe.lock().map_err(|_| "No se pudieron recuperar las filas".to_string())?;
    let mut df_slice = rows.as_ref().unwrap().slice(coerced_index, block_size).clone();
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf).with_json_format(JsonFormat::Json).finish(&mut df_slice).map_err(|e| format!("Error de formato al escribir JSON: {}", e))?;
    let json_rows: Value = serde_json::from_slice(&buf).map_err(|e| format!("Error al estructurar el JSON: {}", e))?;
    Ok(json_rows)
}