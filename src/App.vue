<script setup>
import LimpiezaCSV from "./components/LimpiezaCSV.vue";
import ConfiguracionDiccionarios from "./components/ConfiguracionDiccionarios.vue";
import RevisionMetadatos from "./components/RevisionMetadatos.vue";
import SideBar from "./components/SideBar.vue";
import { ref, watch } from "vue";
import { useGlobalStore } from "./stores/global.js";
//import { invoke } from "@tauri-apps/api/core";

const estadoGlobal = useGlobalStore();
</script>

<template>
  <main>
    <div>
      <SideBar />
    </div>
    <div class="mesa-trabajo">
      <RevisionMetadatos
        v-if="estadoGlobal.seccionSeleccionada === 'metadatos'"
      />
      <ConfiguracionDiccionarios
        v-if="estadoGlobal.seccionSeleccionada === 'diccionario'"
      />
      <LimpiezaCSV v-if="estadoGlobal.seccionSeleccionada === 'limpieza'" />
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
body {
  margin: 0px;
  padding: 0px;
}
.mesa-trabajo {
  position: absolute;
  left: 15vw;
  width: 85%;
  margin-top: 0px;
  padding-top: 0px;
}

@media (max-width: 800px) {
  .mesa-trabajo {
    left: 130px;
    width: calc(100% - 130px);
  }
}
</style>
