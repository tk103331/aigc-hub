<script setup lang="ts">
import { ref } from 'vue'
import { darkTheme, lightTheme } from 'naive-ui'
import { AppProvider } from './components/Application'
import Layout  from './layout/index.vue'
const theme = ref(lightTheme)

</script>
<template>
    <NConfigProvider>
    <AppProvider>
      <Layout>
        
      </Layout>
    </AppProvider>
  </NConfigProvider>
</template>
