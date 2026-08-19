<script setup>
import { ref, watch, computed } from "vue";
import { useGlobalStore } from "../stores/global.js";
const estadoGlobal = useGlobalStore();
</script>
<template>
  <div class="sidebar">
    <div class="identidad p-2">Herramienta de validacion</div>
    <div class="secciones m-t-3">
      <button
        class="boton-chico"
        :class="
          estadoGlobal.seccionSeleccionada === 'metadatos'
            ? 'is-selected'
            : null
        "
        @click="estadoGlobal.actualizarVista('metadatos', 'inicio')"
      >
        Metadatos
      </button>
      <button
        class="boton-chico"
        :class="
          estadoGlobal.seccionSeleccionada === 'diccionario'
            ? 'is-selected'
            : null
        "
        @click="estadoGlobal.actualizarVista('diccionario', 'inicio')"
      >
        Diccionario
      </button>
      <button
        class="boton-chico"
        :class="
          estadoGlobal.seccionSeleccionada === 'limpieza' ? 'is-selected' : null
        "
        @click="estadoGlobal.actualizarVista('limpieza', 'carga')"
      >
        Limpieza de datos
      </button>
    </div>
  </div>
</template>
<style lang="scss" scoped>
.sidebar {
  position: fixed;
  top: 0px;
  left: 0px;
  bottom: 0px;
  background-color: var(--color-neutro-1);
  height: 100vh;
  width: 15vw;
  min-width: 130px;
}
.identidad {
  background-color: pink;
  height: 15vh;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}
button {
  background-color: red;
  width: 100%;
  border-radius: 0%;
  border-bottom: solid 1px green;
}
.is-selected {
  border-left: solid 3px green;
}
</style>
