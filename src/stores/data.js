import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useDataStore = defineStore("data", () => {
  const absolutePath = ref(null);
  const isDataReady = ref(false);
  const blocksInMemmory = 3;
  const blockSize = 30;
  const nthCount = 10; // El nthcount debe ser siempre más pequeño que el block size
  const esquema = ref({
    caracteresCorruptos: null,
    encoding: null,
    columnas: null,
    esquemaColumnas: null,
    totalFilas: null,
    totalColumnas: null,
  });
  const filas = ref({
    lastBlock: 1,
    firstBlock: 1,
    nthLastElement: null,
    nthFirstElement: null,
    bloques: {},
    bloqueConData: {},
  });

  const updatePath = function (pathString) {
    absolutePath.value = pathString;
  };

  const resetearEsquema = function () {
    esquema.value.caracteresCorruptos = null;
    esquema.value.encoding = null;
    esquema.value.columnas = null;
    esquema.value.esquemaColumnas = null;
    esquema.value.totalFilas = null;
    esquema.value.totalColumnas = null;
  };

  const fetchNextRows = async function () {
    // Revisamos si ya existe el key-value pair con el indice indicado
    // y que no sea un array vacío
    const currentIndex = filas.value.lastBlock;
    let fetchedBlocks = Object.keys(filas.value.bloqueConData)
      .filter((n) => filas.value.bloqueConData[n])
      .map((n) => Number(n));

    // Si el indice no existe o es un array vacío,
    // Pedimos los datos para generar un nuevo bloque de key-values
    // y a cada fila le agregamos un indice
    if (!fetchedBlocks.includes(currentIndex)) {
      const newRows = await invoke("fetch_rows", {
        startIndex: filas.value.lastBlock,
        blockSize: blockSize,
      });

      newRows.forEach(
        (d, index) =>
          (d.indice = (filas.value.lastBlock - 1) * blockSize + index),
      );

      //Actualizamos la data de la store
      filas.value.bloques[currentIndex] = newRows;
      filas.value.bloqueConData[currentIndex] = true;
      fetchedBlocks.push(currentIndex);
      fetchedBlocks = fetchedBlocks.sort((a, b) => a - b);

      // Señalamos el nuevo último elemento
      const filas_flat = Object.values(filas.value.bloques).flat();
      if (filas_flat.length - nthCount > 0) {
        filas.value.nthLastElement = filas_flat[filas_flat.length - nthCount];
      } else {
        filas.value.nthLastElement = filas_flat[filas_flat.length - 1];
      }
      filas.value.lastBlock++;

      // Ahora nos aseguramos que no tenemos más bloques de datos de los que queremos
      if (blocksInMemmory < fetchedBlocks.length) {
        const elementToDelete = fetchedBlocks[0];
        filas.value.bloques[elementToDelete] = filas.value.bloques[
          elementToDelete
        ].map((element) => (element = {}));
        filas.value.bloqueConData[elementToDelete] = false;
        filas.value.firstBlock = fetchedBlocks[1];
        filas.value.nthFirstElement =
          filas.value.bloques[filas.value.firstBlock][blockSize - nthCount];
      }
    } else {
      return;
    }
  };

  /**
   * Esta función resetea la información de archivo cada vez que se carga
   * uno nuevo, actualiza el esquema de los datos y también pide el primer
   * bloque de filas
   */
  const readCSV = async function () {
    isDataReady.value = false;
    filas.value.lastBlock = 1;
    filas.value.nthLastElement = null;
    filas.value.bloques = {};
    const data_csv = await invoke("leer_csv", {
      rutaFront: absolutePath.value,
    });
    esquema.value.encoding = data_csv.encoding_aplicado;
    esquema.value.caracteresCorruptos = data_csv.caracteres_corruptos;
    esquema.value.totalFilas = data_csv.total_filas;
    esquema.value.columnas = data_csv.esquema_columnas.map((d) => d.nombre);
    esquema.value.totalColumnas = esquema.value.columnas.length;
    esquema.value.esquemaColumnas = data_csv.esquema_columnas;
    await fetchNextRows();
    isDataReady.value = true;
  };

  return {
    absolutePath,
    isDataReady,
    esquema,
    filas,
    resetearEsquema,
    updatePath,
    readCSV,
    fetchNextRows,
  };
});
