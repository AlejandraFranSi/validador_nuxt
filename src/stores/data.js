import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useDataStore = defineStore("data", () => {
  const absolutePath = ref(null);
  const isDataReady = ref(false);
  const caracteresCorruptos = ref(null);
  const requiereConversion = ref(null);
  const encoding = ref(null);
  const columnas = ref(null);
  const esquemaColumnas = ref(null);
  const filas = ref(null);
  const totalFilas = ref(null);
  const totalColumnas = ref(null);

  const updatePath = function (pathString) {
    absolutePath.value = pathString;
  };
  const fetchData = async function () {
    const data_csv = await invoke("leer_csv", {
      rutaFront: absolutePath.value,
    });
    console.log("El archivo csv: ", data_csv);
    encoding.value = data_csv.nombre_encoding;
    caracteresCorruptos.value = data_csv.caracteresCorruptos;
    filas.value = data_csv.filas;
    totalFilas.value = data_csv.total_filas;
    columnas.value = data_csv.columnas;
    totalColumnas.value = data_csv.columnas.length;
    esquemaColumnas.value = data_csv.esquema_columnas;
    isDataReady.value = true;
  };
  return {
    absolutePath,
    isDataReady,
    columnas,
    filas,
    totalFilas,
    totalColumnas,
    esquemaColumnas,
    updatePath,
    fetchData,
  };
});
