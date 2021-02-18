<template>
    <div class="settings-page">
        <v-container grid-list-md text-xs-center>
            <p class="text-h3">Settings</p>
            <v-row dense>
                <v-col :cols="6">
                    <v-card>
                        <v-card-title primary-title>
                            <div class="headline">Data</div>
                            <v-spacer></v-spacer>
                            <div class="overline"></div>
                        </v-card-title>
                        <v-card-text>
                            <v-subheader>Refresh rate (second)</v-subheader>
                            <v-slider
                                max="10"
                                min="0.5"
                                step="0.5"
                                v-model="settings.refresh_rate"
                                class="align-center"
                            >
                                <template v-slot:append>
                                    <v-text-field
                                        v-model="settings.refresh_rate"
                                        class="mt-0 pt-0"
                                        type="number"
                                        style="width: 60px"
                                    ></v-text-field> </template
                            ></v-slider>
                            <v-subheader>Max items to poll</v-subheader>
                            <v-slider
                                max="200"
                                min="5"
                                step="5"
                                v-model="settings.poll_count"
                                class="align-center"
                            >
                                <template v-slot:append>
                                    <v-text-field
                                        v-model="settings.poll_count"
                                        class="mt-0 pt-0"
                                        type="number"
                                        style="width: 60px"
                                    ></v-text-field> </template
                            ></v-slider>
                        </v-card-text>
                    </v-card>
                </v-col>
                <v-col :cols="6">
                    <v-card>
                        <v-card-title primary-title>
                            <div class="headline">XX</div>
                            <v-spacer></v-spacer>
                            <div class="overline"></div>
                        </v-card-title>
                    </v-card>
                </v-col>
            </v-row>
        </v-container>
    </div>
</template>

<script>
export default {
    name: "Settings",
    data() {
        return {
            settings: {
                refresh_rate: 1.0,
                poll_count: 100,
            },
        };
    },
    methods: {
        updateSettings() {
            this.$store.dispatch(UPDATE_USER, this.currentUser).then(() => {
                // #todo, nice toast and no redirect
                this.$router.push({ name: "home" });
            });
        },
        logout() {
            this.$store.dispatch(LOGOUT).then(() => {
                this.$router.push({ name: "home" });
            });
        },
    },
};
</script>