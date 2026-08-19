<script setup>
import CargaArchivos from "./limpieza/CargaArchivo.vue";
import CompararArchivos from "./limpieza/CompararArchivos.vue";
import EdicionValores from "./limpieza/EdicionValores.vue";
import ValidacionColumnas from "./limpieza/ValidacionColumnas.vue";
import { ref, watch, computed } from "vue";
import { useGlobalStore } from "../stores/global.js";
const estadoGlobal = useGlobalStore();
</script>
<template>
  <h1>Herramientas de limpieza para CSV</h1>
  <h3>¿Cómo funciona esta herramienta?</h3>
  <p>
    Aplica las transformaciones que necesites para que tu base de datos esté más
    limpia.
  </p>
  <div>
    <button @click="estadoGlobal.actualizarVista('limpieza', 'carga')">
      Cargar Archivo
    </button>
    <button @click="estadoGlobal.actualizarVista('limpieza', 'comparar')">
      Comparar
    </button>
    <button @click="estadoGlobal.actualizarVista('limpieza', 'columnas')">
      Validación de columnas
    </button>
    <button @click="estadoGlobal.actualizarVista('limpieza', 'valores')">
      Edición de valores
    </button>
  </div>

  <div>
    <CargaArchivos v-if="estadoGlobal.subseccionSeleccionada === 'carga'" />
    <CompararArchivos
      v-if="estadoGlobal.subseccionSeleccionada === 'comparar'"
    />
    <EdicionValores v-if="estadoGlobal.subseccionSeleccionada === 'valores'" />
    <ValidacionColumnas
      v-if="estadoGlobal.subseccionSeleccionada === 'columnas'"
    />
  </div>
</template>
