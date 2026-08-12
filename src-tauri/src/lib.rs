// Import the libraries and functions we'll use
use chardetng::{EncodingDetector,Iso2022JpDetection, Utf8Detection};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use csv::{Error, ReaderBuilder, StringRecord};
//use encoding_rs::Encoding;
use std::collections::{BTreeMap, BTreeSet,};
use std::{ fs::File};
use std::io::{Read,BufReader};
use serde::{Serialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, leer_csv])
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
    pub col_vals: Vec<String>
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

#[derive(Serialize)]
pub struct ReporteCsv {
    pub nombre_encoding: String,
    pub requiere_conversion: bool,
    pub caracteres_corruptos: Vec<CaracterCorrupto>,
    pub columnas: Vec<String>,
    pub esquema_columnas: Vec<EsquemaColumna>,
    pub filas: Vec<Vec<String>>,
    pub total_filas: usize,

}
#[tauri::command]
fn leer_csv(ruta_front: String) -> Result<ReporteCsv, String>{
    // Esta fucnión debe devolver un objeto con las siguientes keys:
    // caracteres_corruptos, encoding detectado, requiere_conversion, columnas, total_filas, esquema
    // Confirmar que existe la ruta
    let ruta = ruta_front;

    // Leer el archivo como bytes
    // Para leer el archivo creamos un buffer, que es un bloque temporal de memoria que se usa 
    // mientras se mueve de un espacio a otro. Entonces, se leen los bytes del archivo y se van agregando al buffer
    let file: File = File::open(&ruta).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);
    let mut buffer_inicio = vec![0; 4096];
    let bytes_leidos =  reader.read(&mut buffer_inicio).map_err(|e| e.to_string())?;

    let mut file_completo = File::open(&ruta).map_err(|e| e.to_string())?;
    let mut bytes_puros = Vec::new();
    file_completo.read_to_end(&mut bytes_puros).map_err(|e| e.to_string())?;
    //println!("Bytes file completo: {:?}", bytes_puros);

    // Parseamos los bytes_puros como UTF8 y revisamos que no tenga caracteres extraños
    // Se definen las siguientes variables a partir del output que tiene la siguiente forma
    // (Cow<'a, str>, &'static Encoding, bool). El bool regresa true cuando detecta secuencias inválidas 
    let (texto_convertido, _encod, tuviera_errores) = encoding_rs::UTF_8.decode(&bytes_puros);
    let mut mapa_caracteres: BTreeMap<char, BTreeSet<u64>> = BTreeMap::new();
    let mut nombre_encoding = if tuviera_errores {"".to_string()} else { "UTF-8".to_string()} ;
    let requiere_conversion = if tuviera_errores {true} else { false};
    let mut texto_decodificado= texto_convertido.into_owned();
    if tuviera_errores{
        // Constuirmos un detector de encoding en el cual se rechaza la posibilidad de que el resultado sea ISO-2022-JP
        let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
        detector.feed(&buffer_inicio[..bytes_leidos], true);
        //Adivinamos el encoding, permitiendo que la respuesta sea utf8
        let encoding_alternativo = detector.guess(None, Utf8Detection::Allow);
        nombre_encoding = encoding_alternativo.name().to_string();
        // Decodificamos el texto usando el nuevo encoding. La función decode regresa tres valores:
        // El texto,  _encalt, _err
        texto_decodificado = encoding_alternativo.decode(&bytes_puros).0.into_owned();
    }

    for (indice, linea) in texto_decodificado.lines().enumerate() {
        let fila_actual = (indice + 1) as u64;
        for c in linea.chars() {
            if es_caracter_corrupto(c) {
                // Si encontramos un caracter corrupto lo agregamos a mapa_caracteres
                // Se revisa si el elemento ya está en mapa_caracteres, si no está se agrega un valor default con or_default()
                // Y luego se llena ese valor default con el valor de fila_actual
                mapa_caracteres.entry(c).or_default().insert(fila_actual);
            }
        }
    }

    // ¿Por qué es necesario crear una nueva estructura para los caracteres corruptos? 
    //¿Es para poder consumirlo desde el front?
    let caracteres_corruptos: Vec<CaracterCorrupto> = mapa_caracteres.into_iter().map(|(caracter,filas)| CaracterCorrupto {
        caracter: caracter.to_string(),
        filas: filas.into_iter().collect()
    }).collect();

    // Primero vamos a obtener las columnas usando el crate csv y 
    // vamos a quitar espacios vacíos al inicio y final de los nombres
    let mut rdr = ReaderBuilder::new().from_reader(texto_decodificado.as_bytes());
    let columnas: Vec<String>= rdr.headers().map_err(|e| e.to_string()).unwrap().iter().map(|x| x.trim().to_string()).collect();

    // Esta forma de contar las filas se rompe cuando se ecede cierto máximo
    //let total_filas = rdr.records().count();
    // Para evitar crear dos iteradores para obtener las filas, 
    // primero creamos un vector con las filas y luego obtenemos su longitud
    let mut filas = Vec::new();
    let filas_sr: Vec<StringRecord> = rdr.records().map(|record| record.unwrap()).collect();
    let total_filas = filas_sr.len();

    // Obtenemos las filas
    for fila in &filas_sr{
        // Fila es un StringRecord, entonces lo vamos a convertir en un vector con de strings
        let opt:Vec<String>= fila.into_iter().map(|x| x.to_string()).collect();
        filas.push(opt);
    }

    // Ahora vamos a contruir el equema de las columnas que nos indicará 
    //el nombre de cada columna, su tipo y sus valores
    let mut esquema_columnas: Vec<EsquemaColumna>= Vec::new();
    for (i, col) in columnas.iter().enumerate(){
        let mut contenido_col = Vec::new();
        for record in &filas_sr{
            contenido_col.push( record.get(i).unwrap().trim().to_string());
        }
        let tipo_col = parsear_columna(&contenido_col).unwrap();
        esquema_columnas.push(EsquemaColumna{nombre: col.clone(), tipo: tipo_col.to_string(), col_vals: contenido_col})
    }

    Ok(ReporteCsv{nombre_encoding, requiere_conversion, caracteres_corruptos, columnas, esquema_columnas, filas, total_filas, })

}

fn parsear_columna(contenido_columna: &Vec<String>)->Result<String, Error>{
    let mut tipo: String = "".to_string();
    for formato in ["%Y-%m-%d", "%d-%m-%Y","%Y/%m/%d", "%d/%m/%Y"] {
        // A partir de los distintos formaros de fecha dados, se intenta parsear como fecha
        // Se crea un nuevo vector con los casos que fueron exitosos y se comparan las longitudes
        // Si las longitudes coinciden se marca como fecha
        let parsed_as_datetime:Vec<_> = contenido_columna.iter().map(|x| NaiveDate::parse_from_str(x, formato)).collect();
        let is_datetime: Vec<&Result<NaiveDate, ParseError>> = parsed_as_datetime.iter().filter(|x| x.is_ok()).collect();
        if contenido_columna.len() == is_datetime.len(){
            tipo = "Fecha".to_string();
        }
    };

    if tipo == "" {
        // Lo mismo pero parseando como numero
        let parsed_as_number: Vec<Result<i32, std::num::ParseIntError>> = contenido_columna.iter().map(|x| x.parse::<i32>()).collect();
        let is_number: Vec<_> = parsed_as_number.iter().filter(|x| x.is_ok()).collect();
        if contenido_columna.len() == is_number.len(){
            tipo = "Numerico".to_string();
        } else{
            tipo = "Texto".to_string();
        }
    }
    Ok(tipo)
}