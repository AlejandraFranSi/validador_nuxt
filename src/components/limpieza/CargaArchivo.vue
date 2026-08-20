<script setup>
import TablaCSV from "../base/TablaCSV.vue";
import Errores from "../base/Errores.vue";
import * as d3 from "d3";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref, watch, computed, defineEmits } from "vue";
import { useGlobalStore } from "../../stores/global.js";
import { useDataStore } from "../../stores/data.js";
import { invoke } from "@tauri-apps/api/core";
// const invoke = window.__TAURI__.core.invoke;
const estadoGlobal = useGlobalStore();
const estadoData = useDataStore();
const appWindow = getCurrentWindow();
const dropZoneText = ref(null);
const isDataReady = computed(() => estadoData.isDataReady);
const archivoInvalido = ref(false);


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
      //Cada vez que cargamos un archivo, reseteamos la info
      estadoData.resetearEsquema();
      estadoGlobal.actualizarStatusArchivo("Sin archivo cargado");
      // Al soltar el archivo
      archivoInvalido.value = false;
      estadoData.updatePath(event.payload.paths[0]);
      dropZone.classed("dragover", false);
      // Validamos que el archivo efectivamente sea un csv
      if (!estadoData.absolutePath.toLowerCase().endsWith(".csv")) {
        //alert("El archivo debe tener formato CSV.");
        archivoInvalido.value = true;
        return;
      }
      // En caso de que sí sea un csv, actualizamos la interfaz
      dropZoneText.value = `Archivo actual: ${estadoData.absolutePath}`;
      estadoGlobal.actualizarStatusArchivo(
        "Analizando codificación e indizando datos...",
      );
      estadoGlobal.setLoadingFile(true);
      // Leemos el archivo y mostramos la tabla
      const prueba = await estadoData.readCSV();
      // Actualizamos el estatus
      estadoGlobal.setLoadingFile(false);
      estadoGlobal.actualizarStatusArchivo(
        `Listo: ${estadoData.esquema.totalFilas} filas, ${estadoData.esquema.totalColumnas} columnas`,
      );
    } else {
      // En caso de que al final no se haga nada
      dropZone.classed("dragover", false);
    }
  });
});
</script>
<template>
  <div>
        <div>
      <p class="m-1">
        Esta herramienta fue diseñada con el objetivo de facilitar el
        mejoramiento de las bases de datos. Para ello, la herramienta:
      </p>
      <ol>
        <li class="m-1">
          Sugiere nombres que siguen los lineamientos del manual para las
          columnas que puedes editar
        </li>
        <li class="m-1">
          Permite aplicar transformaciones a las columnas para corregir
          características del texto, o transformar el tipo de la columna.
        </li>
        <li class="m-1">
          Permite analizar las columnas textuales que codifican categorías y
          modificar sus valores para homologarlos
        </li>
      </ol>
        <h4>Comienza cargando un archivo</h4>
    </div>
    <div class="flex flex-contenido-centrado">
      <div
        class="dropZone columna-14 borde-redondeado-8 flex flex-contenido-centrado"
        id="dropZone"
      >
        <p class="p-3">{{ dropZoneText }}</p>
      </div>
    </div>
    <div id="state" class="flex flex-contenido-centrado">
      <!--<p><b>Estatus:</b> {{ estadoGlobal.statusArchivo }}</p>-->
      <Errores v-if="archivoInvalido">
        <p class="m-y-1 m-x-2">El archivo debe tener formato CSV.</p>
      </Errores>
      <div v-if="!archivoInvalido && isDataReady" class="m-y-2 p-2 texto-color-confirmacion fondo-color-confirmacion borde borde-redondeado-8 columna-14">
        <p class="m-0">Archivo cargado correctamente</p>
        <ul class="m-0">
          <li class="m-0">Número de filas: {{ estadoData.esquema.totalFilas }}</li>
          <li class="m-0">Número de columnas: {{ estadoData.esquema.totalColumnas }}</li>
          <li class="m-0">Encoding: {{ estadoData.esquema.encoding }}</li>
          <li class="m-0">Caracteres corruptos: {{ estadoData.esquema.caracteresCorruptos }}</li>
        </ul>
      </div>
      <div class="flex m-y-1"  v-if="estadoGlobal.loadingFile">
        <p>"Analizando codificación e indizando datos..."</p>
        <img alt="cargando" src="../../assets/pink-spinner.gif"></img>
      </div>
    </div>
    <TablaCSV v-if="estadoData.isDataReady && !archivoInvalido" />
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
