<script setup>
import TablaCSV from "./TablaCSV.vue";
import * as d3 from "d3";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref, watch, computed } from "vue";
import { useGlobalStore } from "../stores/global.js";
import { useDataStore } from "../stores/data.js";
import { invoke } from "@tauri-apps/api/core";
// const invoke = window.__TAURI__.core.invoke;
const estadoGlobal = useGlobalStore();
const estadoData = useDataStore();
const appWindow = getCurrentWindow();
const dropZoneText = ref(null);
const isDataReady = computed(() => estadoData.isDataReady);

onMounted(() => {
  dropZoneText.value = estadoData.absolutePath
    ? `Archivo actual: ${estadoData.absolutePath}`
    : "Arrastra un csv";
  const dropZone = d3.select("#dropZone");
  appWindow.onDragDropEvent(async (event) => {
    if (event.payload.type === "over") {
      // Cuando aún no se suelta el archivo
      dropZone.classed("dragover", true);
    } else if (event.payload.type === "drop") {
      // Al soltar el archivo
      estadoData.updatePath(event.payload.paths[0]);
      dropZone.classed("dragover", false);
      // Validamos que el archivo efectivamente sea un csv
      if (!estadoData.absolutePath.toLowerCase().endsWith(".csv")) {
        alert("El archivo debe tener formato CSV.");
        return;
      }
      // En caso de que sí sea un csv, actualizamos la interfaz
      dropZoneText.value = `Archivo actual: ${estadoData.absolutePath}`;
      estadoGlobal.actualizarStatusArchivo(
        "Analizando codificación e indizando datos...",
      );
      estadoGlobal.setLoadingFile(true);
      // Leemos el archivo y mostramos la tabla
      const prueba = await estadoData.fetchData();
      // Actualizamos el estatus
      estadoGlobal.setLoadingFile(false);
      estadoGlobal.actualizarStatusArchivo(
        `Listo: ${estadoData.totalFilas} filas, ${estadoData.totalColumnas} columnas`,
      );
      d3.selectAll("button.tools").property("disabled", false);
    } else {
      // En caso de que al final no se haga nada
      dropZone.classed("dragover", false);
    }
  });
});

watch(isDataReady, (nv) => {
  console.log(nv);
});
</script>
<template>
  <div>
    <h1>Herramientas de limpieza para CSVS</h1>
    <h2>¿Cómo funciona esta herramienta?</h2>
    <p>
      Aplica las transformaciones que necesites para que tu base de datos esté
      más limpia.
    </p>
    <p>Comienza cargando un archivo.</p>

    <div class="flex">
      <div
        class="dropZone columna-14 borde-redondeado-8 flex flex-contenido-centrado"
        id="dropZone"
      >
        <p class="p-3">{{ dropZoneText }}</p>
      </div>
    </div>
    <TablaCSV v-if="estadoData.isDataReady" />
  </div>
</template>
<style lang="scss" scoped>
.dropZone {
  background-color: var(--color-neutro-1);
  border: dashed 2px var(--color-neutro-4);
  color: var(--color-neutro-5);
  height: 30vh;
  align-items: center;

  p {
    text-align: center;
    white-space: normal;
    overflow-wrap: break-word;
    width: 100%;
  }
}
</style>
