<script setup>
import TablaCSV from "./TablaCSV.vue";
</script>
<template>
  <h1>Herramienta de columnas</h1>
  <p>
    La herramienta sugiere nombres de columna que puedes editar. Permite aplicar
    transformaciones a las columnas para corregir características del texto, o
    transformar en columnas numéricas o de fecha.
  </p>

  <TablaCSV />

  <h2>Validacion Columnas</h2>
  <p>
    Valida el nombre a usar en cada columna y el tipo de datos que debería de
    contener. La herramineta transformará los datos y ajustará algunos detalles
    para que la columna satisfaga los criterios
  </p>
</template>
