<script setup>
import { computed, onMounted, ref } from "vue";
import { useDataStore } from "../../stores/data.js";
import TablaCSV from "../base/TablaCSV.vue";
import { invoke } from "@tauri-apps/api/core";

const estadoData = useDataStore();
const columnas = computed(() => estadoData.esquema.esquemaColumnas);
const preposiciones = [
  "a",
  "ante",
  "bajo",
  "cabe",
  "con",
  "contra",
  "de",
  "desde",
  "durante",
  "en",
  "entre",
  "hacia",
  "hasta",
  "mediante",
  "para",
  "por",
  "segun",
  "sin",
  "so",
  "sobre",
  "tras",
  "versus",
];

const typeDict = {
  Temporal: "#EE4266",
  Numérica: "#FFFEC2",
  Texto: "#2BB4DE",
};

const promoverCambios = function () {
  let colsDict = [];
  for (const [i, columna] of columnas.value.entries()) {
    const inputNombre = document.getElementById(`nombre-columna-${i}`).value;
    const inputTipo = document.getElementById(`tipo-columna-${i}`).value;
    colsDict.push([columna.nombre, inputNombre, inputTipo]);
  }
  console.log(colsDict);
  invoke("castear_columna", { cols: colsDict });
};
const sugerirNombre = function (nombre) {
  let nombreSugerido = nombre
    .toLowerCase()
    .replace(" ", "_")
    .normalize("NFC")
    .replace(/[\u0300-\u036f]/g, "");
  for (let preposicion of preposiciones) {
    nombreSugerido.replace(preposicion, "");
  }
  return nombreSugerido;
};
</script>
<template>
  <p>
    Valida el nombre a usar en cada columna y el tipo de datos que debería de
    contener. La herramineta transformará los datos y ajustará algunos detalles
    para que la columna satisfaga los criterios
  </p>
  <div class="contenedor-columnas">
    <div class="flex columna-contenida" v-for="(columna, index) in columnas">
      <div>
        <button class="boton-chico boton-secundario columna-1">
          <svg
            width="22px"
            height="22px"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M15.7628 9H7.63719C7.18864 9 6.82501 9.37295 6.82501 9.833V16.5C6.82501 17.8807 7.91632 19 9.26251 19H14.1375C14.784 19 15.404 18.7366 15.8611 18.2678C16.3182 17.7989 16.575 17.163 16.575 16.5V9.833C16.575 9.37295 16.2114 9 15.7628 9Z"
              stroke="#9EC8E4"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              fill-rule="evenodd"
              clip-rule="evenodd"
              d="M14.625 7L13.9191 5.553C13.7541 5.21427 13.4167 5.0002 13.0475 5H10.3526C9.98338 5.0002 9.64596 5.21427 9.48092 5.553L8.77502 7H14.625Z"
              stroke="#9EC8E4"
              stroke-width="1.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <path
              d="M10.8247 12.333C10.8247 11.9188 10.4889 11.583 10.0747 11.583C9.66047 11.583 9.32469 11.9188 9.32469 12.333H10.8247ZM9.32469 15.666C9.32469 16.0802 9.66047 16.416 10.0747 16.416C10.4889 16.416 10.8247 16.0802 10.8247 15.666H9.32469ZM14.0753 12.333C14.0753 11.9188 13.7396 11.583 13.3253 11.583C12.9111 11.583 12.5753 11.9188 12.5753 12.333H14.0753ZM12.5753 15.666C12.5753 16.0802 12.9111 16.416 13.3253 16.416C13.7396 16.416 14.0753 16.0802 14.0753 15.666H12.5753ZM14.625 6.25C14.2108 6.25 13.875 6.58579 13.875 7C13.875 7.41421 14.2108 7.75 14.625 7.75V6.25ZM16.575 7.75C16.9892 7.75 17.325 7.41421 17.325 7C17.325 6.58579 16.9892 6.25 16.575 6.25V7.75ZM8.77501 7.75C9.18923 7.75 9.52501 7.41421 9.52501 7C9.52501 6.58579 9.18923 6.25 8.77501 6.25V7.75ZM6.82501 6.25C6.4108 6.25 6.07501 6.58579 6.07501 7C6.07501 7.41421 6.4108 7.75 6.82501 7.75V6.25ZM9.32469 12.333V15.666H10.8247V12.333H9.32469ZM12.5753 12.333V15.666H14.0753V12.333H12.5753ZM14.625 7.75H16.575V6.25H14.625V7.75ZM8.77501 6.25H6.82501V7.75H8.77501V6.25Z"
              fill="#9EC8E4"
            />
          </svg>
        </button>
      </div>
      <div class="contenedor-nombre columna-7">
        <label :for="`nombre-columna-${index}`" class="m-1"
          >Nombre sugerido para &nbsp
          <span :class="columna.incidencia ? 'verde' : 'rojo'">
            {{ columna.nombre }}</span
          ></label
        >
        <input
          type="text"
          :id="`nombre-columna-${index}`"
          :name="`nombre-columna-${index}`"
          :value="columna.nombre_sugerido"
        />
      </div>
      <div class="contenedor-tipo columna-7">
        <label :for="`tipo-columna-${index}`"
          >Transformar de &nbsp
          <span
            :style="`background-color: ${typeDict[columna.tipo]}; color : #000`"
            >{{ columna.tipo }}
          </span>
          &nbsp a:</label
        >
        <select :name="`nombre-columna-${index}`" :id="`tipo-columna-${index}`">
          <option value="texto">Texto</option>
          <option value="texto-sin-guines">Texto sin guiones</option>
          <option value="texto-minusculas">Texto en minúsculas</option>
          <option value="texto-capitalizado">Texto capitalizado</option>
          <option value="numerico">Numérica</option>
          <option value="fecha">Fecha</option>
          <option value="cordenada">Coordenadas</option>
        </select>
      </div>
    </div>
  </div>
  <button class="boton-primario m-t-2" @click="promoverCambios">
    Promover cambios
  </button>

  <TablaCSV />
</template>
<style scoped lang="scss">
.contenedor-columnas {
  max-height: 30vh;
  overflow-y: auto;
}

.columna-contenida {
  align-items: end;
}

.verde {
  background-color: var(--color-confirmacion-2);
}

.rojo {
  background-color: var(--color-error-2);
}
</style>
