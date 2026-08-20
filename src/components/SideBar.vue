<script setup>
import { useGlobalStore } from "../stores/global.js";
const estadoGlobal = useGlobalStore();
</script>
<template>
  <div class="sidebar">
    <div class="identidad p-2 m-y-7">
      <p>Herramienta de validación</p>
    </div>
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
  height: 100vh;
  width: 15vw;
  background-color: var(--color-secundario-12);
  color: var(--color-neutro-1);
}
.identidad {
  height: 15vh;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  font-size: 18px;
  text-align: center;
}
button {
  background-color: var(--color-secundario-12);
  color: var(--color-neutro-1);
  width: 100%;
  border-radius: 0%;
  border-bottom: solid 1px var(--color-primario-4);
  text-align: left;
}
button:hover {
  background-color: var(--color-secundario-9);
}
.is-selected {
  border-left: solid 5px var(--color-primario-4);
}

@media (max-width: 800px) {
  .sidebar {
    width: 130px;
  }
}
</style>
