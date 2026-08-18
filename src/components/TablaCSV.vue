<script setup>
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useDataStore } from "../stores/data.js";

const estadoData = useDataStore();
const columnas = computed(() => estadoData.esquema.columnas);
const filas = computed(() => estadoData.filas.bloques);
const nthElement = computed(() => estadoData.filas.nthElement);
const nthElementClass = computed(
  () => `fila-${estadoData.filas.nthElement.indice}`,
);
const target = ref(null);
const filasFlat = computed(() =>
  Object.values(estadoData.filas.bloques).flat(),
);
const isFetchingData = ref(false);
const typeDict = {
  Fecha: "#EE4266",
  Numerico: "#FFFEC2",
  Texto: "#2BB4DE",
};

/*function setColor(columnName) {
  let option = estadoData.esquema.esquemaColumnas.filter(
    (col) => col.nombre === columnName,
  )[0];
  return typeDict[option.tipo];
}*/

const fetchNewData = async function (entries, observer) {
  if (entries[0].isIntersecting && !isFetchingData.value) {
    console.log("Hay intersección y podemos pedir datos");
    if (estadoData.esquema.totalFilas > filasFlat.value.length) {
      console.log("Se piden más datos");
      isFetchingData.value = true;
      observer.unobserve(target.value);
      await estadoData.fetchRows();
      await nextTick();
      target.value = document.querySelector(`${nthElementClass.value}`);
      console.log("El nuevo target: ", nthElement.value);
      console.log("El nuevo target: ", nthElementClass.value);

      isFetchingData.value = false;
    } else {
      observer.unobserve(target.value);
    }
  }
};

onMounted(async () => {
  const options = {
    root: null,
    rootMargin: "0px",
    scrollMargin: "0px",
    threshold: 0.1,
  };

  const observer = new IntersectionObserver(fetchNewData, options);
  target.value = document.querySelector(`.${nthElementClass.value}`);
  if (target) {
    observer.observe(target.value);
  }
  //console.log(nthElementClass.value);
});

/*watch(filasFlat, async () => {
  //observer.observe(target.value);
});*/
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
            <th v-for="columna in columnas">
              {{ columna }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="fila in filasFlat" :class="`fila-${fila.indice}`">
            <td v-for="columna in columnas">{{ fila[columna] }}</td>
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
