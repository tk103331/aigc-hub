<template>
    <div>
        <div v-for="app in appList" :key="app.name" style="margin-bottom: 10px;">
            <NCard>
            <div>
                <NAvatar :src="app.icon"></NAvatar>
                <NButton text @click="openApp(app)">
                    <span style="font-size: 28px;font-weight: 700;line-height: 34px;margin-left: 10px;">
                        {{ app.name }} 
                    </span>
                </NButton>
                
                <div style="display: inline-block;float: right;">
                    <NIcon :component="Person"></NIcon>
                    <span>{{ app.author }}</span>
                </div>
            </div>
            <span>{{ app.desc }}</span>
        </NCard>
        </div>

    </div>


</template>
<script setup lang="ts">
import { NAvatar, NCard, NIcon, NTag } from 'naive-ui';
import {Person} from '@vicons/ionicons5';
import { invoke } from '@tauri-apps/api';

const appList = ref([]);
invoke('load_app_list').then((res) => {
    console.log(res);
    appList.value = res;
});

function openApp(app) {
    invoke('open_app', { "appName": app.name });
}

</script>