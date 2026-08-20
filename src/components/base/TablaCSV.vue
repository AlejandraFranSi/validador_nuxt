<script setup>
import { computed, nextTick, onMounted, ref } from "vue";
import { useDataStore } from "../../stores/data.js";

const estadoData = useDataStore();
const columnas = computed(() => estadoData.esquema.columnas);
const filas = computed(() => estadoData.filas.bloques);
const filasFlat = computed(() => Object.values(filas.value).flat());
const nthLastElementClass = computed(
  () => `fila-${estadoData.filas.nthLastElement.indice}`,
);
const target = ref(null);
const observerLast = ref(null);
const isFetchingData = ref(false);
const typeDict = {
  Fecha: "#EE4266",
  Numerico: "#FFFEC2",
  Texto: "#2BB4DE",
};

const fetchNextRows = async function (entries, observer) {
  if (entries[0].isIntersecting && !isFetchingData.value) {
    if (estadoData.esquema.totalFilas > filasFlat.value.length) {
      isFetchingData.value = true;
      observer.unobserve(target.value);
      await estadoData.fetchNextRows();
      isFetchingData.value = false;
      await nextTick();
      target.value = document.querySelector(`.${nthLastElementClass.value}`);
      if (target.value) {
        observer.observe(target.value);
      }
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
  observerLast.value = new IntersectionObserver(fetchNextRows, options);
  target.value = document.querySelector(`.${nthLastElementClass.value}`);

  if (target.value) {
    observerLast.value.observe(target.value);
  }
});
</script>
<template>
  <div class="componente-tabla">
    <h4>Vista de los datos</h4>
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
.componente-tabla {
  margin-top: 0px;
  margin-bottom: 30px;
  height: 90vh;
}
.contenedor-tabla {
  max-width: 100%;
  max-height: 70vh;
}
</style>
