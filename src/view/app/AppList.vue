<template>
    <div>
        <div v-for="app in appList" :key="app.name" style="margin-bottom: 10px;">
            <NCard>
            <div>
                <NAvatar :src="app.icon"></NAvatar>
                <NButton text>
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
 
    const appList = [
        {
            name: 'ChatGPT',
            icon: 'https://chat.openai.com/favicon-32x32.png',
            url: 'https://chat.openai.com',
            author: 'OpenAI',
            desc: 'An artificial intelligence chatbot developed by OpenAI and built on top of OpenAI\'s GPT-3.5 and GPT-4 families of large language models (LLMs) '
        },
        {
            name: 'Poe',
            icon: 'https://poe.com/favicon.ico',
            url: 'https://poe.com',
            author: 'Poe',
            desc: 'A platform that lets people ask questions, get instant answers, and have back-and-forth conversations with AI.'
        },
        {
            name: 'Cursor',
            icon: 'https://www.cursor.so/favicon.ico',
            url: 'https://www.cursor.so',
            author: 'Cursor',
            desc: 'An editor made for programming with AI'
        },
    ];
</script>