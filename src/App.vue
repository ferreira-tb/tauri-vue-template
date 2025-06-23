<script setup lang="ts">
import { onMounted } from 'vue';
import { showWindow } from '@/commands';
import { useColorMode } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { handleError, onKeyDown } from '@tb-dev/vue';

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(async () => {
  try {
    await showWindow();
  } catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <main class="h-screen w-screen select-none">
    <div class="size-full overflow-hidden p-0">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
