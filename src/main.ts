import 'vue-sonner/style.css';
import '@/assets/style/base.css';
import '@/assets/style/main.css';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { onError } from '@/lib/error';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  TauriPluginPinia({
    autoStart: true,
    saveOnChange: true,
  }),
);

setCurrentApp(app);
setErrorHandler(onError, app);

app.use(router);
app.use(pinia);

app.mount('#app');
