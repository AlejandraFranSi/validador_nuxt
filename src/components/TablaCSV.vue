<script setup>
import { computed } from "vue";
import { useDataStore } from "../stores/data.js";

const estadoData = useDataStore();
const columnas = computed(() => estadoData.columnas);
const filas = computed(() => estadoData.filas);
const typeDict = {
  Fecha: "#EE4266",
  Numerico: "#FFFEC2",
  Texto: "#2BB4DE",
};

function setColor(columnName) {
  let option = estadoData.esquemaColumnas.filter(
    (col) => col.nombre === columnName,
  )[0];
  return typeDict[option.tipo];
}
</script>
<template>
  <div>
    <h2>Vista de los datos</h2>
    <p>
      Tipo de columna:
      <span
        v-for="tipo in Object.keys(typeDict)"
        :style="{ 'background-color': typeDict[tipo] }"
        >{{ tipo }},
      </span>
    </p>
    <div class="contenedor-tabla">
      <table>
        <thead>
          <tr>
            <th
              v-for="columna in columnas"
              :style="{ 'background-color': setColor(columna) }"
            >
              {{ columna }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="fila in filas">
            <td v-for="valor in fila">{{ valor }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<style lang="scss" scoped>
.contenedor-tabla {
  max-width: 95%;
  max-height: 80vh;
}
</style>
