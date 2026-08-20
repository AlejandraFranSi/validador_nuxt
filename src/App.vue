<script setup>
import LimpiezaCSV from "./components/LimpiezaCSV.vue";
import ConfiguracionDiccionarios from "./components/ConfiguracionDiccionarios.vue";
import RevisionMetadatos from "./components/RevisionMetadatos.vue";
import SideBar from "./components/SideBar.vue";
import { useGlobalStore } from "./stores/global.js";

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

  /*--color-secundario-1: #fffcf4ff;
  --color-secundario-2: #e5e6d5ff;
  --color-secundario-3: #cad6c6ff;
  --color-secundario-4: #b1c6b8ff;
  --color-secundario-5: #9bb9acff;
  --color-secundario-6: #83ab9eff;
  --color-secundario-7: #74a196ff;
  --color-secundario-8: #5b9089ff;
  --color-secundario-9: #588e87ff;
  --color-secundario-10: #3f7f7bff;
  --color-secundario-11: #1e6b68ff;
  --color-secundario-12: #065a5aff;

  --color-primario-1: #2cb4b4ff;
  --color-primario-2: #229496;
  --color-primario-3: #fb5d01ff;
  --color-primario-4: #ff711f;*/
}
body {
  margin: 0px;
  padding: 0px;
  font-size: 14px;
  background-color: var(--color-neutro-0);
}
.mesa-trabajo {
  position: absolute;
  left: 15vw;
  width: 85%;
  height: calc(100vh + 100px);
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
