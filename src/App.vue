<template>
  <div id="app">
    <b-container fluid>
      <router-view />
    </b-container>
  </div>
</template>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

body {
  background-color: #1c1c1c;
}

.container-fluid {
  padding-left: 0px;
  padding-right: 0px;
}
</style>
