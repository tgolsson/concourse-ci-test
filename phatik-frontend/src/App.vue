<template>
    <v-app id="#inspire">
        <v-navigation-drawer v-model="drawer" app clipped>
            <v-list dense>
                <router-link to="/app">
                    <v-list-item link>
                        <v-list-item-action>
                            <v-icon>mdi-view-dashboard</v-icon>
                        </v-list-item-action>
                        <v-list-item-content>
                            <v-list-item-title>Dashboard</v-list-item-title>
                        </v-list-item-content>
                    </v-list-item>
                </router-link>
                <router-link to="/settings">
                    <v-list-item link>
                        <v-list-item-action>
                            <v-icon>mdi-cog</v-icon>
                        </v-list-item-action>
                        <v-list-item-content>
                            <v-list-item-title>Settings</v-list-item-title>
                        </v-list-item-content>
                    </v-list-item>
                </router-link>
            </v-list>
        </v-navigation-drawer>

        <v-app-bar app clipped-left>
            <v-app-bar-nav-icon
                @click.stop="drawer = !drawer"
            ></v-app-bar-nav-icon>
            <v-toolbar-title>Phatik</v-toolbar-title>

            <v-spacer></v-spacer>
            <v-switch v-model="$vuetify.theme.dark" hide-details> </v-switch>

            <v-progress-linear
                :active="loading"
                :indeterminate="true"
                absolute
                bottom
                color="deep-purple accent-4"
            ></v-progress-linear>
        </v-app-bar>
        <v-main>
            <router-view></router-view>
        </v-main>
        <v-footer app>
            <span>&copy; {{ new Date().getFullYear() }}</span>
        </v-footer>
    </v-app>
</template>



<script>
export default {
    name: "App",
    data() {
        return {
            drawer: true,
        };
    },
    computed: {
        loading() {
            console.log(this.$store.state.in_wait);
            return this.$store.state.in_wait;
        },
    },
};
</script>