import Vue from 'vue'
import App from './App.vue'
import VueMaterial from 'vue-material'
import 'vue-material/dist/vue-material.min.css'
import 'vue-material/dist/theme/default.css'
import vuetify from './plugins/vuetify';
import 'roboto-fontface/css/roboto/roboto-fontface.css'
import '@mdi/font/css/materialdesignicons.css'
import VueTimeago from 'vue-timeago'

import VueRouter from "vue-router";
import Events from "./Events.vue"
import Settings from "./Settings.vue"
import { TimeAgo } from 'vue2-timeago'
import Vuex from 'vuex'

Vue.use(VueMaterial);
Vue.use(VueRouter);
Vue.use(Vuex)

Vue.use(TimeAgo, {
    name: 'timeago', // Component name, `Timeago` by default
    locale: 'en', // Default locale
})

const routes = [
    { path: '/', redirect: '/app' },
    { path: "/app", component: Events },
    { path: "/settings", component: Settings },
];

const router = new VueRouter({
    routes, // short for `routes: routes`
});


const store = new Vuex.Store({
    state: {
        settings: {
            updateFrequency: 1.0,
            limit: 100,
        },
        in_wait: false,
    },
    mutations: {
        begin_long_poll(state) {
            // mutate state
            state.in_wait = true;
        },
        end_long_poll(state) {
            // mutate state
            state.in_wait = false;
        }
    }
})

new Vue({
    router,
    vuetify,
    store: store,
    data() {
        return {
            drawer: true,
        }
    },
    created() {
        this.$vuetify.theme.dark = true;
    },

    render: (h) => h(App)

}).$mount('#app')
