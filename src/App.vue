<script setup lang="ts">
import { onMounted } from 'vue';
import { showWindow } from '@/commands';
import { useColorMode } from '@vueuse/core';
import { Sonner } from '@tb-dev/vue-components';
import { exit } from '@tauri-apps/plugin-process';
import { handleError, onKeyDown } from '@tb-dev/vue';

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => showWindow().err());
</script>

<template>
  <main class="fixed inset-0 select-none">
    <Sonner />
    <div class="relative size-full overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
