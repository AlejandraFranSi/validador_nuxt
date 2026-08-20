<script setup>
import CargaArchivos from "./limpieza/CargaArchivo.vue";
import CompararArchivos from "./limpieza/CompararArchivos.vue";
import EdicionValores from "./limpieza/EdicionValores.vue";
import ValidacionColumnas from "./limpieza/ValidacionColumnas.vue";
import { ref, watch, computed } from "vue";
import { useGlobalStore } from "../stores/global.js";
const estadoGlobal = useGlobalStore();
const isFileLoaded = computed(() =>
  estadoGlobal.statusArchivo === "Sin archivo cargado" ? false : true,
);
</script>
<template>
  <div class="vista-gral m-x-3">
    <h1 class="m-y-3">Herramientas de limpieza para CSV</h1>
    <div class="control-seccion m-y-1">
      <button
        :class="
          estadoGlobal.subseccionSeleccionada === 'carga' ? 'is-selected' : null
        "
        @click="estadoGlobal.actualizarVista('limpieza', 'carga')"
      >
        Cargar Archivo
      </button>
      <!--<button
        class="boton-chico"
        :class="
          estadoGlobal.subseccionSeleccionada === 'comparar'
            ? 'is-selected'
            : null
        "
        @click="estadoGlobal.actualizarVista('limpieza', 'comparar')"
        :disabled="!isFileLoaded"
      >
        Comparar
      </button> -->
      <button
        :class="
          estadoGlobal.subseccionSeleccionada === 'columnas'
            ? 'is-selected'
            : null
        "
        @click="estadoGlobal.actualizarVista('limpieza', 'columnas')"
        :disabled="!isFileLoaded"
      >
        Validación de columnas
      </button>
      <button
        :class="
          estadoGlobal.subseccionSeleccionada === 'valores'
            ? 'is-selected'
            : null
        "
        @click="estadoGlobal.actualizarVista('limpieza', 'valores')"
        :disabled="!isFileLoaded"
      >
        Edición de valores
      </button>
    </div>

    <div>
      <CargaArchivos v-if="estadoGlobal.subseccionSeleccionada === 'carga'" />
      <CompararArchivos
        v-if="estadoGlobal.subseccionSeleccionada === 'comparar'"
      />
      <EdicionValores
        v-if="estadoGlobal.subseccionSeleccionada === 'valores'"
      />
      <ValidacionColumnas
        v-if="estadoGlobal.subseccionSeleccionada === 'columnas'"
      />
    </div>
  </div>
</template>
<style lang="scss" scoped>
.control-seccion {
  width: 100%;
}
button {
  border-radius: 0%;
}
.is-selected {
  border-bottom: solid 5px var(--color-primario-4);
}
</style>
