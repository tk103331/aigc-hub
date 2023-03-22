<script setup lang="ts">
import { ref } from 'vue'
import { darkTheme, lightTheme, NH2, NImage, NLayout, NLayoutContent, NLayoutHeader, NLayoutSider, NSpace } from 'naive-ui'
const theme = ref(lightTheme)
const menus = [
  {
    label: 'Dashboard',
    key: 'dashboard',
  },
  {
    label: 'ChatGPT',
    key: 'chatgpt',
  },
  {
    label: 'ChatGPT',
    key: 'chatgpt',
  },
  {
    label: 'ChatGPT',
    key: 'chatgpt',
  },
  {
    label: 'ChatGPT',
    key: 'chatgpt',
  },
  {
    label: 'ChatGPT',
    key: 'chatgpt',
  }
];
</script>
<template>
  <n-space vertical>
 
    <n-layout>
      <n-layout-header bordered>
        Header Header Header
      </n-layout-header>
      <n-layout has-sider>
        <n-layout-sider
          bordered
          show-trigger="bar"
          collapse-mode="width"
          :collapsed-width="64"
          :width="240"
          :native-scrollbar="false"
          style="max-height: 320px"
        >
          <n-menu
            :collapsed-width="64"
            :collapsed-icon-size="22"
            :options="menus"
          />
        </n-layout-sider>
        <n-layout style="max-height: 320px" />
      </n-layout>
      <n-layout-footer bordered>
        Footer Footer Footer
      </n-layout-footer>
    </n-layout>
  </n-space>
</template>
