<script setup lang="ts">
import { ref } from 'vue'
import { darkTheme, lightTheme } from 'naive-ui'

const theme = ref(lightTheme)

</script>
<template>
  <n-config-provider :theme="theme">
    <n-space vertical>
      
      <n-layout>
        <n-layout>
          <n-layout-header bordered>
            <n-menu mode="horizontal" />
            <n-avatar src="./assets/logo.png"></n-avatar>
            <span>AIGC-App</span>
          </n-layout-header>
          <n-layout has-sider>
            <n-layout-sider
              bordered
              collapse-mode="width"
              :collapsed-width="64"
              :width="240"
              show-trigger
            >
              <n-menu
                :collapsed-width="64"
                :collapsed-icon-size="22"
                :options="[]"
              />
            </n-layout-sider>
            <n-layout>
              <n-button-group>
                <n-button @click="theme = darkTheme">深色</n-button>
                <n-button @click="theme = lightTheme">浅色</n-button>
              </n-button-group>
            </n-layout>
          </n-layout>
        </n-layout>
      </n-layout>
    </n-space>
  </n-config-provider>
</template>
