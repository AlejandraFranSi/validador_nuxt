<script setup>
import * as d3 from "d3";
import TablaCSV from "../base/TablaCSV.vue";
import ReporteArchivo from "./ReporteArchivo.vue";
import TarjetaError from "../base/TarjetaError.vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref, computed} from "vue";
import { useDataStore } from "../../stores/data.js";
import { useGlobalStore } from "../../stores/global.js";

const appWindow = getCurrentWindow();
const dropZoneText = ref(null);
const estadoData = useDataStore();
const estadoGlobal = useGlobalStore();
const estaCargando = computed(() => estadoData.dataStatus.isLoading);
const seLeyoArchivo = computed(() => estadoData.dataStatus.wasFetchingSuccesfull);
const erroresLectura = computed(() => estadoData.dataStatus.fetchingError);
const validacionNombre = ref([
  {
    "leyenda": "¿El nombre del archivo está en español?",
    "grupo": "nombreEspaniol",
    "valor": true
  },
  {
    "leyenda": "¿El nombre del archivo es descriptivo del conjunto de datos?",
    "grupo": "nombreDescriptivo",
    "valor": true
  },
  {
    "leyenda": "¿El nombre del archivo incluye la temporalidad de los datos?",
    "grupo": "nombreTemporal",
    "valor": true
  }
])

const irAValidacion = function(){
  console.log("Se guardan los errores");
  estadoGlobal.actualizarVista('limpieza', 'columnas')
}

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
      dropZoneText.value = `Archivo actual: ${estadoData.absolutePath}`;
      await estadoData.readCSV();
    } else {
      // En caso de que al final no se haga nada
      dropZone.classed("dragover", false);
    }
  });
});
</script>
<template>
  <div>
    <!-- La presentacion-->
    <div id="presentacion">
      <p class="m-1">
        Esta herramienta fue diseñada con el objetivo de facilitar el
        mejoramiento de las bases de datos. Para ello, la herramienta:
      </p>
      <ol>
        <li class="m-1">
          Sugiere nombres que siguen los lineamientos del manual para las
          columnas y el archivo.
        </li>
        <li class="m-1">
          Permite aplicar transformaciones a las columnas para corregir
          características del texto, o transformar el tipo de la columna.
        </li>
        <li class="m-1">
          Permite analizar las columnas textuales que codifican categorías y
          modificar sus valores para homologarlos.
        </li>
      </ol>
        <h4>Comienza cargando un archivo</h4>
    </div>
    <!-- El dropdown -->
    <div id="drag-and-drop" class="flex flex-contenido-centrado">
      <div
        class="dropZone columna-14 borde-redondeado-8 flex flex-contenido-centrado"
        id="dropZone"
      >
        <p class="p-3">{{ dropZoneText }}</p>
      </div>
    </div>
    <!-- El spinner-->
    <div id="spinner" class="flex m-t-4" v-if="estaCargando">
      <div class="flex flex-contenido-centrado columna-16">
          <img alt="cargando" src="../../assets/pink-spinner.gif" height="100px"></img>
      </div>
      <p class="columna-16">Analizando codificación e indizando datos...</p>
    </div>
    <!-- Si no se pudo abrir el archivo por cualquier razón-->
    <div v-if="!estaCargando && seLeyoArchivo  === false"
      class="tarjeta-estado flex flex-contenido-centrado" >
      <TarjetaError>
        <p>{{erroresLectura}}</p>
      </TarjetaError> 
    </div>
    <!-- Si la lectura del archivo fue exitosa-->
    <div v-if="!estaCargando && seLeyoArchivo  === true">
      <div id="state" class="flex flex-contenido-centrado">
        <ReporteArchivo class="tarjeta-estado"/>
      </div>
      <TablaCSV id="tabla-carga"/>
      <div class="m-t-8">
        <h3>A partir de la vista de datos, responde lo siguiente:</h3>
          <SisdaiBotonesRadioGrupo
          v-for="pregunta in validacionNombre"
          :key="pregunta.grupo"
          :leyenda="pregunta.leyenda"
          >
            <SisdaiBotonRadio
              v-model="pregunta.valor"
              etiqueta="Si"
              :value="`${true}`"
              :name="pregunta.grupo"
            />
            <SisdaiBotonRadio
              v-model="pregunta.valor"
              etiqueta="No"
              :value="`${false}`"
              :name="pregunta.grupo"
            />
          </SisdaiBotonesRadioGrupo>
      </div>
      <button class="boton-primario" @click="irAValidacion">Siguiente</button>
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
