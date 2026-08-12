import { ref } from "vue";
import { defineStore } from "pinia";

export const useGlobalStore = defineStore("global", () => {
  const vistaSeleccionada = ref("carga");
  const statusArchivo = ref("Sin archivo cargado");
  const loadingFile = ref(false);

  const actualizarVista = function (nuevaVista) {
    vistaSeleccionada.value = nuevaVista;
  };

  const actualizarStatusArchivo = function (nuevoStatus) {
    statusArchivo.value = nuevoStatus;
  };

  const setLoadingFile = function (bool) {
    loadingFile.value = bool;
  };

  return {
    vistaSeleccionada,
    statusArchivo,
    loadingFile,
    actualizarVista,
    actualizarStatusArchivo,
    setLoadingFile,
  };
});
