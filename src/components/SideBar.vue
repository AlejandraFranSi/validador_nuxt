<script setup>
import { ref, watch, computed } from "vue";
import { useGlobalStore } from "../stores/global.js";
const estadoGlobal = useGlobalStore();
</script>
<template>
  <div class="sidebar">
    <button
      class="boton-primario m-2"
      type="button"
      id="cargarTool"
      data-bs-toggle="button"
      @click="estadoGlobal.actualizarVista('carga')"
    >
      Cargar archivo
    </button>
    <button
      class="boton-primario m-2 tools"
      type="button"
      id="validacionTool"
      data-bs-toggle="button"
      disabled
      @click="estadoGlobal.actualizarVista('validacion')"
    >
      Validación de columnas
    </button>
    <button
      class="boton-primario m-2 tools"
      type="button"
      id="categosTool"
      data-bs-toggle="button"
      disabled
      @click="estadoGlobal.actualizarVista('edicion')"
    >
      Edición de valores
    </button>
    <hr />
    <button
      class="boton-secundario m-2 tools"
      type="button"
      id="comparaTool"
      disabled
    >
      Comparar
    </button>
    <hr />
    <button
      class="boton-archivo m-2 tools"
      type="button"
      id="descargaTool"
      disabled
    >
      Exportar a CSV
    </button>
    <button
      class="boton-archivo m-2 tools"
      type="button"
      id="subirTool"
      disabled
    >
      Subir archivo
    </button>

    <div id="stata" class="m-y-1 m-x-2">
      <p><b>Estatus</b></p>
      <div class="flex m-y-1">
          <img v-if="estadoGlobal.loadingFile" alt="cargando" src="../assets/pink-spinner.gif"></img>
          <p class="columna-12">{{ estadoGlobal.statusArchivo }}</p>
      </div>
    </div>
  </div>
</template>
<style lang="scss" scoped>

#stata {
  font-size: smaller;
  p{
    margin: 0px;
  }
  img{
        height: 16px;
        width: 16px;
      }
}
.sidebar {
  background-color: var(--color-neutro-1);
  height: 100vh;
}

.boton-archivo {
  background-color: var(--color-secundario-8);
  color: var(--color-secundario-1);
}
.boton-archivo:hover {
  background-color: var(--color-secundario-6);
}
.boton-archivo:disabled {
  background-color: var(--color-neutro-2);
  color: var(--color-neutro-4);
}
</style>
