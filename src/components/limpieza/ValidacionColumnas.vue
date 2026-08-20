<script setup>
import { useDataStore } from "../../stores/data.js";
import TablaCSV from "../base/TablaCSV.vue";

const estadoData = useDataStore();
console.log(estadoData.esquema);
</script>
<template>
  <p>
    Valida el nombre a usar en cada columna y el tipo de datos que debería de
    contener. La herramineta transformará los datos y ajustará algunos detalles
    para que la columna satisfaga los criterios
  </p>
  <div class="contenedor-columnas">
    <div class="flex" v-for="(columna, index) in estadoData.esquema.columnas">
      <div class="contenedor-nombre columna-8">
        <label :for="`nombre-columna-${index}`"
          >Nombre sugerido para {{ columna }}</label
        >
        <input type="text" :name="`nombre-columna-${index}`" />
      </div>
      <div class="contenedor-tipo columna-8">
        <label :for="`tipo-columna-${index}`"
          >Columna tipo: {{ columna }}. Transformar a</label
        >
        <select :name="`nombre-columna-${index}`">
          <option value="texto">Texto</option>
          <option value="texto-sin-guines">Texto sin guiones</option>
          <option value="texto-minusculas">Texto en minúsculas</option>
          <option value="texto-capitalizado">Texto capitalizado</option>
          <option value="numerico">Numérica</option>
          <option value="fecha">Fecha</option>
          <option value="cordenada">Coordenadas</option>
          <option value="eliminar">Eliminar columna</option>
        </select>
      </div>
    </div>
  </div>
  <button class="boton-primario m-t-2">Promover cambios</button>

  <TablaCSV />
</template>
<style scoped lang="scss">
.contenedor-columnas {
  max-height: 30vh;
  overflow-y: auto;
}
</style>
