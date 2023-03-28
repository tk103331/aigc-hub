<script setup lang="ts">
import { ref } from 'vue'
import AppProvider from '../components/AppProvider.vue';
import { NIcon, NTabPane, NTabs } from 'naive-ui'
import AppList from './app/AppList.vue';
import ToolList from './tool/ToolList.vue';
import Config from './config/Config.vue';
import { Apps, Hammer, Settings } from '@vicons/ionicons5';

</script>
<template>
  <AppProvider>
    <div style="padding:0 20px;">
      <NTabs type="line" animated>
        <NTabPane name="app">
          <template #tab>
            <NIcon :component="Apps"></NIcon>
            <span> App</span>
          </template>
          <AppList></AppList>
        </NTabPane>
        <NTabPane name="tool">
          <template #tab>
            <NIcon :component="Hammer"></NIcon>
            <span> Tool</span>
          </template>
          <ToolList></ToolList>
        </NTabPane>
        <template #suffix>
          <NIcon :component="Settings"></NIcon>
        </template>

      </NTabs>
    </div>
  </AppProvider>
</template>
