import { ref } from "vue";
import { defineStore } from "pinia";

export const useGlobalStore = defineStore("global", () => {
  const seccionSeleccionada = ref("limpieza");
  const subseccionSeleccionada = ref("carga");

  const actualizarVista = function (seccion, subseccion) {
    seccionSeleccionada.value = seccion;
    subseccionSeleccionada.value = subseccion;
  };

  return {
    seccionSeleccionada,
    subseccionSeleccionada,
    actualizarVista,
  };
});
