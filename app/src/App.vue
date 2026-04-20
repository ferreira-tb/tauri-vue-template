<script setup lang="ts">
import { onMounted } from 'vue';
import { showWindow } from '@/commands';
import { useColorMode } from '@vueuse/core';
import Loading from '@/components/Loading.vue';
import { Sonner } from '@tb-dev/vue-components';
import { exit } from '@tauri-apps/plugin-process';
import { handleError, onKeyDown, useBreakpoints } from '@tb-dev/vue';

const { md } = useBreakpoints();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => showWindow().err());
</script>

<template>
  <main class="fixed inset-0 select-none pb-safe">
    <Sonner :position="md ? 'bottom-left' : 'top-center'" />
    <div class="relative size-full overflow-hidden">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <Suspense>
            <component :is="Component" />
            <template #fallback>
              <Loading class="absolute inset-0" />
            </template>
          </Suspense>
        </template>
      </RouterView>
    </div>
  </main>
</template>
