<script setup>
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useDataStore } from "../stores/data.js";

const estadoData = useDataStore();
const columnas = computed(() => estadoData.esquema.columnas);
const filas = computed(() => estadoData.filas.bloques);
const nthElement = computed(() => estadoData.filas.nthElement);
const nthElementClass = ref(null);
const target = ref(null);
const filasFlat = computed(() => {
  if (Object.keys(Object.values(estadoData.filas.bloques)).length === 1) {
    return Object.values(estadoData.filas.bloques).flat();
  } else {
    return Object.values(estadoData.filas.bloques).flat();
  }
});
const typeDict = {
  Fecha: "#EE4266",
  Numerico: "#FFFEC2",
  Texto: "#2BB4DE",
};

function setColor(columnName) {
  let option = estadoData.esquema.esquemaColumnas.filter(
    (col) => col.nombre === columnName,
  )[0];
  return typeDict[option.tipo];
}

function buildElementClass(elementArray) {
  return elementArray
    .join("-")
    .slice(0, 20)
    .toLowerCase()
    .replaceAll(" ", "_")
    .replaceAll(".", "")
    .replaceAll(",", "");
}

const fetchNewData = async function (entries, observer) {
  if (estadoData.esquema.totalFilas > filasFlat.value.length) {
    console.log("Se piden más datos");
    await estadoData.fetchRows();
    nthElementClass.value = buildElementClass(estadoData.filas.nthElement);
    observer.unobserve(target.value);
    target.value = document.querySelector(`tr.${nthElementClass.value}`);
    observer.observe(target.value);
  } else {
    observer.unobserve(target.value);
  }
};

onMounted(async () => {
  nthElementClass.value = buildElementClass(estadoData.filas.nthElement);
  const options = {
    root: null,
    rootMargin: "0px",
    scrollMargin: "0px",
    threshold: 1.0,
  };

  const observer = new IntersectionObserver(fetchNewData, options);
  target.value = document.querySelector(`tr.${nthElementClass.value}`);
  observer.observe(target.value);
});
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
          <tr v-for="fila in filasFlat" :class="`${buildElementClass(fila)}`">
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
  max-height: 60vh;
}
</style>
