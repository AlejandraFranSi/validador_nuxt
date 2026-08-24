import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useDataStore = defineStore("data", () => {
  const absolutePath = ref(null);
  const isDataReady = ref(false);
  const wasFetchingSuccesfull = ref(null);
  const fetchingError = ref(null);
  const blocksInMemmory = 3;
  const blockSize = 20;
  const nthCount = 3; // El nthcount debe ser siempre más pequeño que el block size
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

  /**
   * Esta función se encarga de pedir las filas del bloque siguiente
   * y se asegura que no se tengan más bloques de filas que los señalados
   */
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
          filas.value.bloques[filas.value.firstBlock][nthCount];
      } else {
        filas.value.nthFirstElement = filas_flat[nthCount];
      }
    }
  };

  /**
   * Esta función se encarga de pedir las filas del bloque anterior
   * y se asegura que no se tengan más bloques de filas que los señalados
   */
  const fetchPreviousRows = async function () {
    // Solo pedimos el bloque anterior cuando no estamos en el primer bloque
    if (filas.value.firstBlock - 1 > 0) {
      filas.value.firstBlock -= 1;
      const prevRows = await invoke("fetch_rows", {
        startIndex: filas.value.firstBlock,
        blockSize: blockSize,
      });
      prevRows.forEach(
        (d, index) =>
          (d.indice = (filas.value.firstBlock - 1) * blockSize + index),
      );
      filas.value.bloques[filas.value.firstBlock] = prevRows;
      filas.value.bloqueConData[filas.value.firstBlock] = true;
      filas.value.nthFirstElement =
        filas.value.bloques[filas.value.firstBlock][nthCount];

      // Ahora nos aseguramos que no tenemos más bloques de datos de los que queremos
      let fetchedBlocks = Object.keys(filas.value.bloqueConData)
        .filter((n) => filas.value.bloqueConData[n])
        .map((n) => Number(n))
        .sort((a, b) => a - b);

      if (blocksInMemmory < fetchedBlocks.length) {
        const elementToDelete = fetchedBlocks[fetchedBlocks.length - 1];
        filas.value.bloques[elementToDelete] = filas.value.bloques[
          elementToDelete
        ].map((element) => (element = {}));
        filas.value.bloqueConData[elementToDelete] = false;
        filas.value.lastBlock = [fetchedBlocks.length - 2];
        filas.value.nthLastElement =
          filas.value.bloques[filas.value.lastBlock][blockSize - nthCount];
      }
    }
  };
  /**
   * Esta función resetea la información de archivo cada vez que se carga
   * uno nuevo, actualiza el esquema de los datos y también pide el primer
   * bloque de filas
   */
  const readCSV = async function () {
    wasFetchingSuccesfull.value = null;
    fetchingError.value = null;
    isDataReady.value = false;
    filas.value.lastBlock = 1;
    filas.value.firstBlock = 1;
    filas.value.nthLastElement = null;
    filas.value.nthFirstElement = null;
    filas.value.bloques = {};
    filas.value.bloqueConData = {};
    try {
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
      wasFetchingSuccesfull.value = true;
      isDataReady.value = true;
    } catch (error) {
      fetchingError.value = error;
      wasFetchingSuccesfull.value = false;
      isDataReady.value = true;
    }
  };

  return {
    absolutePath,
    isDataReady,
    wasFetchingSuccesfull,
    fetchingError,
    esquema,
    filas,
    resetearEsquema,
    updatePath,
    readCSV,
    fetchNextRows,
    fetchPreviousRows,
  };
});
