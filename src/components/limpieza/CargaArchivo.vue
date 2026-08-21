<script setup>
import * as d3 from "d3";
import TablaCSV from "../base/TablaCSV.vue";
import TarjetaError from "../base/TarjetaError.vue";
import TarjetaConfirmacion from "../base/TarjetaConfirmacion.vue";
import TarjetaAlerta from "../base/TarjetaAlerta.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref, computed} from "vue";
import { useGlobalStore } from "../../stores/global.js";
import { useDataStore } from "../../stores/data.js";
//import { invoke } from "@tauri-apps/api/core";

const estadoGlobal = useGlobalStore();
const estadoData = useDataStore();
const appWindow = getCurrentWindow();
const dropZoneText = ref(null);
const isDataReady = computed(() => estadoData.isDataReady);
const archivoInvalido = ref(false);
const listaCaracteresCorruptos = ref([])

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
      // Al soltar el archivo y reseteamos la data store
      dropZoneText.value = ``;
      estadoGlobal.actualizarStatusArchivo("Sin archivo cargado");
      estadoData.resetearEsquema();
      archivoInvalido.value = false;
      estadoData.updatePath(event.payload.paths[0]);
      dropZone.classed("dragover", false);
      // Validamos que el archivo efectivamente sea un csv
      if (!estadoData.absolutePath.toLowerCase().endsWith(".csv")) {
        archivoInvalido.value = true;
        return;
      }
      // En caso de que sí sea un csv, actualizamos la interfaz y los datos
      dropZoneText.value = `Archivo actual: ${estadoData.absolutePath}`;
      estadoGlobal.setLoadingFile(true);
      await estadoData.readCSV();
      if(estadoData.wasFetchingSuccesfull === true){
        estadoData.esquema.caracteresCorruptos.map((d) => {
          if(d.caracter.trim().length > 0) {
            listaCaracteresCorruptos.value.push(d.caracter)}
        })
      }

      estadoGlobal.actualizarStatusArchivo("Listo");
      estadoGlobal.setLoadingFile(false);
    } else {
      // En caso de que al final no se haga nada
      dropZone.classed("dragover", false);
    }
  });
});
</script>
<template>
  <div>
    <div id="presentacion">
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
    <div id="drag-and-drop" class="flex flex-contenido-centrado">
      <div
        class="dropZone columna-14 borde-redondeado-8 flex flex-contenido-centrado"
        id="dropZone"
      >
        <p class="p-3">{{ dropZoneText }}</p>
      </div>
    </div>
    <div id="spinner" class="flex m-t-4"  v-if="estadoGlobal.loadingFile && !estadoData.isDataReady">
      <div class="flex flex-contenido-centrado columna-16">
          <img alt="cargando" src="../../assets/pink-spinner.gif" height="100px"></img>
      </div>
      <p class="columna-16">Analizando codificación e indizando datos...</p>
    </div>
    <div v-if="estadoData.wasFetchingSuccesfull === true">
      <div id="state" class="flex flex-contenido-centrado">
        <TarjetaError v-if="archivoInvalido">
          <p class="m-y-1 m-x-2"> 
            <span class="pictograma-alerta" aria-hidden="true"></span>
            El archivo debe tener formato CSV.
          </p>
        </TarjetaError>
        <TarjetaConfirmacion v-if="!archivoInvalido && isDataReady && estadoData.esquema.encoding === 'UTF-8'  && listaCaracteresCorruptos.length == 0" class="tarjeta-estado">
          <p>Archivo cargado correctamente</p>
          <ul>
            <li>Número de filas: {{ estadoData.esquema.totalFilas }}</li>
            <li>Número de columnas: {{ estadoData.esquema.totalColumnas }}</li>
            <li>Encoding: {{ estadoData.esquema.encoding }}</li>
            <li >No se encontaron caracteres corruptos</li>
          </ul>
        </TarjetaConfirmacion>
        <TarjetaAlerta v-else-if="!archivoInvalido && isDataReady" class="tarjeta-estado">
          <p>Archivo cargado correctamente</p>
          <ul>
            <li>Número de filas: {{ estadoData.esquema.totalFilas }}</li>
            <li>Número de columnas: {{ estadoData.esquema.totalColumnas }}</li>
            <li>Encoding: {{ estadoData.esquema.encoding }} 
              <span class="pictograma-alerta" aria-hidden="true"></span>
            </li>
            <li v-if="listaCaracteresCorruptos.length > 0">
              Caracteres corruptos: {{ listaCaracteresCorruptos.join(', ')}} 
              <span class="pictograma-alerta" aria-hidden="true"></span>
            </li>
            <li v-else>
              No se encontraron caracteres corruptos
            </li>
          </ul>
        </TarjetaAlerta>
      </div>
      <TablaCSV id="tabla-carga" v-if="estadoData.isDataReady && !archivoInvalido" />
    </div>
    <div v-if="estadoData.wasFetchingSuccesfull === false" 
      class="tarjeta-estado flex flex-contenido-centrado" >
      <TarjetaError>
        <p>{{estadoData.fetchingError}}</p>
      </TarjetaError>
    </div>
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

.tarjeta-estado{
  p,
  ul,
  ol,
  li {
  margin: 0px;
  }
}

#spinner{
  p{
    text-align: center;
  }
}
</style>
