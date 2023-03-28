<template>
  <n-config-provider>
    <n-dialog-provider>
      <n-notification-provider>
      <n-message-provider>
          <slot name="default"></slot>
      </n-message-provider>
      </n-notification-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { NConfigProvider, NDialogProvider, NNotificationProvider, NMessageProvider } from 'naive-ui';
export default defineComponent({
  name: 'AppProvider',
  components: {
  NConfigProvider,
  NDialogProvider,
  NNotificationProvider,
  NMessageProvider,
  },
  setup() {
  return {};
  },
});
</script>