import SisdaiComponentes from "@centrogeomx/sisdai-componentes/src/index";
import "@centrogeomx/sisdai-css";
import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();

app.use(SisdaiComponentes).use(pinia);
app.mount("#app");
