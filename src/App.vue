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
@import url("https://fonts.googleapis.com/css2?family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap");
@import url("https://fonts.googleapis.com/css2?family=Bevan:ital@0;1&family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap");
body[data-perfil="predeterminada"],
:root {
  --tipografia-familia: "Nunito", sans-serif;
  --tipografia-tamanio: 16px;
  --color-neutro-7: #131415;
  --color-neutro-6: #2a2b2e;
  --color-neutro-5: #44474b;
  --color-neutro-4: #61656b;
  --color-neutro-3: #899396;
  --color-neutro-2: #c9cfcf;
  --color-neutro-1: #eaecec;
  --color-neutro-0: #ffffff;

  --color-secundario-1: #edf9fcff;
  --color-secundario-2: #ddedf3ff;
  --color-secundario-3: #cee3eaff;
  --color-secundario-4: #bad4e0ff;
  --color-secundario-5: #a4c5d4ff;
  --color-secundario-6: #8bb3c7ff;
  --color-secundario-7: #7aa6beff;
  --color-secundario-8: #6496b3ff;
  --color-secundario-9: #5389a9ff;
  --color-secundario-10: #357399ff;
  --color-secundario-11: #1b5c89ff;
  --color-secundario-12: #003e68ff;

  /*
  --color-primario-4: #2cb4b4ff;
  --color-primario-3: #229496;
  --color-primario-1: #fb5d01ff;
  --color-primario-2: #ea7a25;
  */
  --color-primario-3: #e83151;
  --color-primario-4: #f391a1;
  --color-primario-1: #d8f6b7;
  --color-primario-2: #b2ec6f;

  --color-error-2: #f9c8cf;
  --color-confirmacion-2: #c2f5b5;
}
body {
  margin: 0px;
  padding: 0px;
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
