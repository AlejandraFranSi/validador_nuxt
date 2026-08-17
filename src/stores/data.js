import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useDataStore = defineStore("data", () => {
  const absolutePath = ref(null);
  const isDataReady = ref(false);
  const esquema = ref({
    caracteresCorruptos: null,
    requiereConversion: null,
    encoding: null,
    columnas: null,
    esquemaColumnas: null,
    totalFilas: null,
    totalColumnas: null,
  });
  const filas = ref({
    currentBlock: 1,
    blockSize: 100,
    nthElement: null,
    bloques: {},
  });

  const updatePath = function (pathString) {
    absolutePath.value = pathString;
  };

  const updateCurrentBlock = function () {
    filas.value.currentBlock++;
  };

  const fetchRows = async function () {
    const rowsBlock = await invoke("fetch_rows", {
      startIndex: filas.value.currentBlock,
      blockSize: filas.value.blockSize,
    });

    if (!Object.keys(filas.value.bloques).includes(filas.value.currentBlock)) {
      rowsBlock.forEach(
        (d, index) => (d.indice = filas.value.currentBlock + index),
      );
      filas.value.bloques[filas.value.currentBlock] = rowsBlock;
    }
    const lastBlock = Number(Object.keys(filas.value.bloques).at(-1));
    filas.value.nthElement =
      filas.value.bloques[lastBlock][filas.value.blockSize - 5];
    updateCurrentBlock();
  };

  const readCSV = async function () {
    isDataReady.value = false;
    // Solicitamos el esquema de los datos
    const data_csv = await invoke("leer_csv", {
      rutaFront: absolutePath.value,
    });
    // Actualizamos la variable del esquema de los datos
    esquema.value.encoding = data_csv.encoding_aplicado;
    esquema.value.caracteresCorruptos = data_csv.caracteres_corruptos;
    esquema.value.totalFilas = data_csv.total_filas;
    esquema.value.columnas = data_csv.esquema_columnas.map((d) => d.nombre);
    esquema.value.totalColumnas = esquema.value.columnas.length;
    esquema.value.esquemaColumnas = data_csv.esquema_columnas;

    // Vamos a pedir el primer bloque de columnas
    await fetchRows();
    // Actualizamos el estado de los datos
    isDataReady.value = true;
  };

  return {
    absolutePath,
    isDataReady,
    esquema,
    filas,
    updatePath,
    readCSV,
    fetchRows,
  };
});
