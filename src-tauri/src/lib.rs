// Import the libraries and functions we'll use
use chardetng::{EncodingDetector,Iso2022JpDetection, Utf8Detection};
use encoding_rs::*;
use polars::prelude::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet,};
use std::{ fs::{File}};
use std::io::{Read, BufReader, BufRead, Cursor};
use std::sync::Mutex;
use serde::{Serialize};
use tauri::State;
use regex::Regex;
//use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;
use std::sync::OnceLock;
//use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
         .manage(ContenedorDatos { 
            dataframe: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![leer_csv, fetch_rows, castear_columna])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Debug)]
pub struct ValidacionCadena {
    cadena: String,
    sugerido: String,
    incidencia: bool,
    errores: Vec<String>,
}
#[derive(Serialize, Debug)]
pub struct CaracterCorrupto {
    pub caracter: String,
    pub filas: Vec<u64>
}
#[derive(Serialize, Debug)]
pub struct EsquemaColumna {
    pub nombre: String,
    pub nombre_sugerido: String,
    pub tipo: String,
    pub incidencia: bool,
    pub errores: Vec<String>,
}

#[derive(Serialize)]
pub struct ReporteCsv {
    pub nombre_archivo: ValidacionCadena,
    pub encoding_aplicado: String,
    pub caracteres_corruptos: Vec<CaracterCorrupto>,
    pub total_filas: usize,
    pub total_columnas: usize,
    pub esquema_columnas: Vec<EsquemaColumna>,
    pub nombres_columnas_repetidas: bool,
    pub hay_filas_repetidas: bool,
    //pub line_sep: String,
    pub sep_by_coma: bool,
    pub null_rows: usize,
}

pub struct ContenedorDatos {
    pub dataframe: Mutex<Option<DataFrame>>,
}

/**
 * Esta función recibe una cadena y hace las siguientes revisiones
 * 1. Quita espacios iniciales y finales
 * 2. Transforma todo a minúsculas
 * 3. Cambia las ñ por ni
 * 4. Quita acentos
 * 5. Cambia espacios por guiones bajos
 * 6. Quita artículos y preposiciones
 * 7. Quita caracteres especiales
 * Regresa un arreglo con el nombre original, el nombre sugerido,
 * si estos coinciden y la lista de errores
 */
fn validar_cadena(cadena_arg: &str) -> ValidacionCadena{
    let mut errores: Vec<String> = Vec::new();
    static RE_NO_ALFA: OnceLock<Regex> = OnceLock::new();
    let re_no_alfa = RE_NO_ALFA.get_or_init(|| Regex::new(r"[^a-z0-9]").unwrap());
    let prohibidas = ["el","la","los","las","un","una","unos","unas","a","que",
                                "ante","bajo","cabe","con","contra","de","del","durante",
                                "en","entre","mediante","para","segun","por",
                                "sin","so","sobre","tras","versus","y","o","e","u"];

    let cadena = cadena_arg.to_string();
    let sin_espacios_iniciales: String = cadena.trim().to_string();
    if sin_espacios_iniciales.len() != cadena.len(){
        errores.push("Incluye espacios vacíos al inicio o al final".to_string());
    }
    let en_minusculas: String = sin_espacios_iniciales.to_lowercase().to_string();
    if sin_espacios_iniciales != en_minusculas{
        errores.push("Incluye mayúsculas".to_string());
    }

    let sin_enie = en_minusculas.replace('ñ', "ni");
    if sin_enie != en_minusculas{
        errores.push("Incluye la letra ñ".to_string());
    }
    let sin_acentos = sin_enie.nfd().filter(|c|!('\u{0300}'..='\u{036f}').contains(c)).collect::<String>();
    if sin_acentos != sin_enie{
        errores.push("Incluye acentos".to_string());
    }
    let sin_espacios = sin_acentos.replace(" ", "_");
    if sin_espacios != sin_acentos {
        errores.push("Incluye espacios".to_string());
    }

    let palabras: Vec<&str> = sin_espacios
        .split('_')
        .filter(|p| !p.is_empty() && !prohibidas.contains(p))
        .collect();
    let sin_articulos = palabras.join("_");
    if sin_articulos != sin_espacios {
        errores.push("Incluye artículos o preposiciones".to_string());
    }
    let sin_especiales = re_no_alfa.replace_all(&sin_articulos, "_").into_owned();
    if sin_articulos != sin_especiales {
        errores.push("Incluye otros carácteres especiales".to_string());
    }
    let sugerido = sin_especiales;
    let incidencia = sugerido == cadena;
    ValidacionCadena{cadena, sugerido, incidencia, errores}
}

/**
 * Esta función identifica si un caracter es válido en UTF8 o no
 */
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
 * Esta función intenta hacer la transformación de una columna con valores
 * de un tipo a otro tipo. También intenta cambiar el nombre de la columna.
 */
#[tauri::command]
fn castear_columna(cols: Vec<(&str, &str, &str)>){
    println!("{:?}", cols);
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
    let directorio: Vec<&str> = ruta.split('\\').collect();
    let nombre = directorio[directorio.len() - 1].replace(".csv", "");
    let nombre_archivo = validar_cadena(&nombre);    
    let file = File::open(&ruta).map_err(|_| "No se pudo abrir el archivo solicitado. Confirma que la ruta exista.".to_string())?;

    let mut partial_reader = BufReader::new(file);
    let mut partial_bytes = vec![0; 4096];
    let _reading = partial_reader.read(&mut partial_bytes).map_err(|_| "No se pudo leer el archivo.".to_string());

    let tuviera_errores = encoding_rs::UTF_8.decode(&partial_bytes).2;
    let encoder = if tuviera_errores {
        let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
        detector.feed(&partial_bytes, true);
        detector.guess(None, Utf8Detection::Allow)
        } else {
            encoding_rs::UTF_8
        };
    let encoding_aplicado = encoder.name().to_string();
    let mut decoder = encoder.new_decoder(); // Este es el traductor dinámico que en teoría no romperá bytes


    let file_completo = File::open(&ruta).map_err(|_| "No se pudo abrir el archivo. Intenta de nuevo".to_string()).unwrap();
    let mut file_as_bytes: BufReader<File> = BufReader::new(file_completo);
    let mut contenido_final = Vec::new();
    let mut mapa_caracteres: BTreeMap<char, BTreeSet<u64>> = BTreeMap::new();
    
    let mut buffer_intermedio = [0u8; 2048];
    let mut texto_convertido = String::new();
    // Pasamos el contenido del archivo al encoding correcto
    loop {
        let bytes_leidos = file_as_bytes.read(&mut buffer_intermedio).map_err(|_| "No se pudo iterar sobre el archivo.")?;
        let capacidad_necesaria = decoder.max_utf8_buffer_length(bytes_leidos).expect("El cálculo de capacidad se desbordó");
        texto_convertido.reserve(capacidad_necesaria);
        let es_ultimo_bloque = bytes_leidos == 0;
        
        let (_resultado_decode, _consumidos, _hubo_errores) = decoder.decode_to_string(
            &buffer_intermedio[..bytes_leidos],
            &mut texto_convertido,
            es_ultimo_bloque,
        );

        contenido_final.extend_from_slice(texto_convertido.as_bytes());
        texto_convertido.clear();

        if es_ultimo_bloque {
            break;
        }
    }

    for (indice, linea) in contenido_final.lines().enumerate() {
        let line = linea.map_err(|_|"Ocurrió un error al iterar sobre las filas. Intentalo de nuevo.")?;
        let fila_actual = (indice + 1) as u64;
        for c in line.chars() {
                if es_caracter_corrupto(c) {
                    mapa_caracteres.entry(c).or_default().insert(fila_actual);
                }
        }
    }


    let caracteres_corruptos: Vec<CaracterCorrupto> = mapa_caracteres.into_iter().map(|(caracter,filas)| CaracterCorrupto {
        caracter: caracter.to_string(),
        filas: filas.into_iter().collect()
    }).collect();

    // Construimos el df
    let cursor = Cursor::new(&contenido_final);
    let mut esquema_columnas: Vec<EsquemaColumna> = Vec::new();  
    let mut df = CsvReader::new(cursor).with_options(
        CsvReadOptions::default()
            .with_has_header(true)
            .map_parse_options(|opts| opts.with_separator(b','))        
        ).finish().map_err(|_| "No se pudo construir el DataFrame".to_string())?;

    let sep_by_coma = !(df.width() <= 1);

    if df.height() == 0 {
        let new_cursor = Cursor::new(&contenido_final);
        df = CsvReader::new(new_cursor).with_options(
        CsvReadOptions::default()
            .with_has_header(true)
            .map_parse_options(|parse_options| parse_options.with_eol_char(b'\r'))
        ).finish().map_err(|_| "No se pudo construir el DataFrame".to_string())?;
    }
    if df.height() == 0 {
       return Err("No se pudo leer correctamente el archivo. Verifica que no tenga columnas sin nombre ni encabezados".to_string())
    }

    // Ahora vamos a intentar castear las columnas del df
    let nombres: Vec<String> = df
    .get_column_names()
    .iter()
    .map(|s| s.to_string())
    .collect();

    //let df_sin_nulls = df.drop_nulls(Some(&nombres)).map_err(|_| "Fracasó la búsqueda de nulos".to_string())?;
    let mut df_nulls = df.clone().lazy()
        .filter(
            nombres
                .iter()
                .map(|c| col(c).is_null())
                .reduce(|acc, e| acc.and(e))
                .ok_or_else(|| "La lista de columnas está vacía".to_string())?
                .not()
        )
        .collect()
        .map_err(|_| "Fracasó la búsqueda de nulos".to_string())?;
    
    let total_filas = df_nulls.height();
    let null_rows = df.height() - df_nulls.height();
    let are_rows_unique = df_nulls.is_duplicated().map_err(|_| "No se pudo comparar las filas.".to_string())?;
    let repeticiones = are_rows_unique.into_series().value_counts(true, true, PlSmallStr::from_str("valores"), true).map_err(|_| "No se pudo comparar las filas.".to_string())?;
    let hay_filas_repetidas = repeticiones.height() > 1;
    let nombres_repetidos: Vec<&String> =  nombres.iter().filter(|x| x.contains("_duplicated_")).collect();
    let nombres_columnas_repetidas:bool = if nombres_repetidos.iter().len() > 0 { true} else {false};
    let total_columnas = nombres.len();

    for nombre in nombres {
        let propiedades = validar_cadena(&nombre);
        let nombre_sugerido = propiedades.sugerido;
        let incidencia: bool = propiedades.incidencia;
        let errores: Vec<String> = propiedades.errores;
        let column = df_nulls.column(&nombre).unwrap().clone();
        let mut tipo: String;

        let parsed_as_datetime = column.as_materialized_series().date();
        if parsed_as_datetime.is_ok() {
            let parsed_column = column.as_materialized_series().date().unwrap().clone().into_column();
            df_nulls.replace(&nombre, parsed_column);
            tipo = "Temporal".to_string();
        } else {
            let parsed_as_float = column.as_materialized_series().f64();
            if parsed_as_float.is_ok(){
                let parsed_column = parsed_as_float.unwrap().clone().into_column();
                df_nulls.replace(&nombre, parsed_column);
                tipo = "Numérica".to_string();
            } else { 
                let parsed_as_int = column.as_materialized_series().i64();
                if parsed_as_int.is_ok(){
                    let parsed_column = parsed_as_int.unwrap().clone().into_column();
                    df_nulls.replace(&nombre, parsed_column);
                    tipo = "Numérica".to_string();
                } else { 
                    tipo = "Texto".to_string();
                }
            }
        }
        esquema_columnas.push(EsquemaColumna{nombre, nombre_sugerido, incidencia, tipo, errores});
    }

    let mut guardado = state.dataframe.lock().map_err(|_| "Error al bloquear el estado")?;
    *guardado = Some(df_nulls);

    Ok(ReporteCsv{nombre_archivo, encoding_aplicado, caracteres_corruptos, total_filas, total_columnas, esquema_columnas, nombres_columnas_repetidas, hay_filas_repetidas, sep_by_coma, null_rows})

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