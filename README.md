# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Acerca de este proyecto

Este proyecto busca ser un primer boceto de una app que integre todos los pasos involucrados en la revisión de datos para su publicación. Así, busca imitar las funcionalidades del [validador de bases de datos](https://github.com/irvingfisica/validador_app) y [diccionarios](https://github.com/irvingfisica/diccionarios) desarrollados por Irving Morales.

Es un primer acercamiento a Rust, por lo que los crates y funciones empleadas son mucho menos sofisticadas que las originales. También es un primer acercamiento al desarrollo de aplicaciones integrando un front-end con un back-end, en el cual se emplea Vue y bibliotecas como [sisdai-css](https://github.com/irvingfisica/validador_app) para el front-end.

## Prerrequisitos

1. Tauti CLI:
2. Rust
3. Vite
4. Vue

## Compilación:

1. Clonar el repositorio

```
https://github.com/AlejandraFranSi/validador_vue
```

2. Abrir la carpeta del proyecto

```
cd <ruta-repo>
```

3.1. Para levantarlo localmente

```
npm run tauri dev
```

3.2. Para desplegarlo

```
npm run tauri build
```
