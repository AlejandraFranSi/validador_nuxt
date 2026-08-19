import { ref } from "vue";
import { defineStore } from "pinia";

export const useGlobalStore = defineStore("global", () => {
  const seccionSeleccionada = ref("limpieza");
  const subseccionSeleccionada = ref("carga");
  const statusArchivo = ref("Sin archivo cargado");
  const loadingFile = ref(false);

  const actualizarVista = function (seccion, subseccion) {
    seccionSeleccionada.value = seccion;
    subseccionSeleccionada.value = subseccion;
  };

  const actualizarStatusArchivo = function (nuevoStatus) {
    statusArchivo.value = nuevoStatus;
  };

  const setLoadingFile = function (bool) {
    loadingFile.value = bool;
  };

  return {
    seccionSeleccionada,
    subseccionSeleccionada,
    statusArchivo,
    loadingFile,
    actualizarVista,
    actualizarStatusArchivo,
    setLoadingFile,
  };
});
