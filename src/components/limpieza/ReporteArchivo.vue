<script setup>
import { computed } from "vue";
import { useDataStore } from "../../stores/data.js";
const estadoData = useDataStore();
const numFilas = computed(() => estadoData.esquema.totalFilas);
const numColumnas = computed(() => estadoData.esquema.totalColumnas);
const encoding = computed(() => estadoData.esquema.encoding);
const conColumnasRepetidas = computed(
  () => estadoData.esquema.conNombresColumnasRepetidos,
);
const conFilasRepetidas = computed(() => estadoData.esquema.hayFilasRepetidas);
const comaEsSeparador = computed(() => estadoData.esquema.sep_coma);
const filasVacias = computed(() => estadoData.esquema.filas_vacias);
const caracteresCorruptos = computed(() =>
  estadoData.esquema.caracteresCorruptos
    .map((d) => d.caracter)
    .filter((d) => d !== "")
    .join(", "),
);
const erroresEnNombre = computed(() =>
  estadoData.esquema.infoArchivo.errores.join(", "),
);
const conObervaciones = computed(() => {
  if (
    erroresEnNombre.value.length > 0 ||
    caracteresCorruptos.value.length > 0 ||
    encoding.value !== "UTF-8" ||
    conColumnasRepetidas.value === true ||
    conFilasRepetidas.value === true
  ) {
    return true;
  } else {
    return false;
  }
});
</script>

<template>
  <div
    class="m-y-2 p-2 borde borde-redondeado-8 columna-14"
    :class="
      conObervaciones
        ? 'texto-color-alerta fondo-color-alerta'
        : 'texto-color-confirmacion fondo-color-confirmacion'
    "
  >
    <h4
      v-if="conFilasRepetidas || conColumnasRepetidas || !comaEsSeparador"
      class="m-y-1"
    >
      Se recomienda revisar el archivo
    </h4>
    <h4 v-else class="m-y-1">Archivo cargado correctamente</h4>
    <ul>
      <li>Número de filas: {{ numFilas }}</li>
      <li>Número de columnas: {{ numColumnas }}</li>
      <li>Encoding: {{ encoding }}</li>
      <li v-if="conFilasRepetidas">Se encontraron filas repetidas.</li>
      <li v-if="filasVacias > 0">Se encontraron y eliminaron filas vacías.</li>
      <li v-if="erroresEnNombre.length > 0">
        El nombre del archivo no sigue el formato establecido:
        {{ erroresEnNombre }}
      </li>
      <li v-else>El nombre del archivo sigue el formato establecido.</li>
      <li v-if="conColumnasRepetidas">
        Se encontraron columnas sin nombre o con nombres repetidos.
      </li>
      <li v-if="!comaEsSeparador">
        El archivo no usa "," como separador de caracteres.
      </li>
      <li v-if="caracteresCorruptos.length > 0">
        Caracteres corruptos:
        <span class="pictograma-alerta" aria-hidden="true">
          {{ caracteresCorruptos }}</span
        >
      </li>
      <li v-else>No se encontraron caracteres corruptos</li>
    </ul>
  </div>
</template>
<style scoped lang="scss">
p,
ul,
ol,
li {
  margin: 0px;
}
</style>
