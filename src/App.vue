<script setup>
import VistaCarga from "./components/VistaCarga.vue";
import VistaEdicion from "./components/VistaEdicion.vue";
import VistaValidacion from "./components/VistaValidacion.vue";
import SideBar from "./components/SideBar.vue";
import { ref, watch } from "vue";
import { useGlobalStore } from "./stores/global.js";
//import { invoke } from "@tauri-apps/api/core";

const estadoGlobal = useGlobalStore();
</script>

<template>
  <main class="container flex">
    <div class="columna-3">
      <SideBar />
    </div>
    <div class="mesa-trabajo columna-13">
      <VistaCarga v-if="estadoGlobal.vistaSeleccionada === 'carga'" />
      <!-- <VistaValidacion v-if="estadoGlobal.vistaSeleccionada === 'validacion'" />
      <VistaEdicion v-if="estadoGlobal.vistaSeleccionada === 'edicion'" />-->
    </div>
  </main>
</template>

<style lang="scss">
body[data-perfil="predeterminada"],
:root {
  --color-neutro-7: #131415;
  --color-neutro-6: #2a2b2e;
  --color-neutro-5: #44474b;
  --color-neutro-4: #61656b;
  --color-neutro-3: #899396;
  --color-neutro-2: #c9cfcf;
  --color-neutro-1: #eaecec;
  --color-neutro-0: #ffffff;
}

.boton-validacion {
  background-color: var(--color-primario-3);
  color: var(--color-secundario-1);
}
.boton-archivo {
  background-color: var(--color-secundario-11);
  color: var(--color-secundario-1);
}
.boton-archivo:disabled {
  background-color: var(--color-secundario-4);
}
</style>
