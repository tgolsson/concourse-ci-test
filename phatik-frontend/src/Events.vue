
<template>
    <v-container>
        <div v-if="events.length == 0">
            <h3>Welcome to Phatik!</h3>
            <p>
                Phatik is like Twitter for bots, which is much like the real
                Twitter. The goal is to provide a "live feed" of what
                automations are doing in a birds-eye view, on the opposite end
                of the spectrum from logs or application traces.
            </p>
            <p>
                <v-btn color="primary" large @click="show_help_dialog = true"
                    ><v-icon>help</v-icon>Help</v-btn
                >
            </p>
        </div>
        <div v-else>
            <v-card flat>
                <v-container>
                    <v-row class="child-flex">
                        <v-toolbar>
                            <v-combobox
                                v-model="active_tags"
                                :items="all_tags"
                                label="Search by tags"
                                chips
                                clearable
                                multiple
                                hide-details
                                single-line
                            >
                                <template
                                    v-slot:selection="{ attrs, item, selected }"
                                >
                                    <v-chip
                                        v-bind="attrs"
                                        :input-value="selected"
                                        :value="item"
                                        outlined
                                        filter
                                        @click="remove(item)"
                                    >
                                        <strong>#{{ item }}</strong>
                                    </v-chip>
                                </template>
                            </v-combobox>
                        </v-toolbar>
                    </v-row>
                </v-container>
            </v-card>
            <v-container grid-list-md>
                <v-layout row wrap>
                    <v-flex
                        xl3
                        lg4
                        md6
                        xs12
                        v-for="(item, index) in active_events"
                        v-bind:key="`event-${index}`"
                    >
                        <card
                            :event="item"
                            ref="active_tags"
                            v-bind:active_tags.sync="active_tags"
                        ></card>
                    </v-flex>
                </v-layout>
            </v-container>
        </div>

        <v-dialog v-model="show_help_dialog" max-width="500px">
            <v-card>
                <v-card-title>Help</v-card-title>
                <v-card-text>
                    To add data to Phatik, either use one of our
                    <a href="https://github.com/tgolsson/phatik.git"
                        >prebuilt libraries</a
                    >, or make a POST request to the endpoint `/api/status`.
                </v-card-text>
                <v-card-actions>
                    <v-btn
                        color="primary"
                        text
                        @click.stop="show_help_dialog = false"
                        >Close</v-btn
                    >
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-container>
</template>

<script>
import Card from "./Card";
export default {
    name: "App",
    components: { Card },
    data: function () {
        return {
            events: [],
            awaiting_message: false,
            timer: null,

            all_tags: [],
            awaiting_tags_message: false,
            tags_timer: null,

            connection: null,
            last_id: -1,
            show_help_dialog: false,
            active_tags: [],

            retry_iteration: 0,
        };
    },

    computed: {
        active_events: function () {
            if (this.active_tags.length == 0) {
                return this.events;
            } else {
                return this.events.filter((item) =>
                    this.active_tags.every((val) => item.tags.includes(val))
                );
            }
        },
    },

    created: function () {
        this.reconnect(0);
    },

    methods: {
        remove(tag) {
            while (this.active_tags.indexOf(tag) !== -1) {
                this.active_tags.splice(this.active_tags.indexOf(tag), 1);
            }
        },

        fetchEventsList() {
            if (!this.awaiting_message) {
                this.connection.send(
                    JSON.stringify({
                        request: {
                            last_id: this.last_id,
                        },
                    })
                );
                this.awaiting_message = true;
                this.$store.commit("begin_long_poll");
            }
        },

        refreshTagsList() {
            if (!this.awaiting_tags_message) {
                this.connection.send(
                    JSON.stringify({
                        tag_request: {},
                    })
                );
                this.awaiting_tags_message = true;
            }
        },

        cancelAutoUpdate() {
            clearInterval(this.timer);
            clearInterval(this.tags_timer);
        },

        reconnect(current_delay) {
            console.log("Starting connection to WebSocket Server");
            let protocol =
                window.location.protocol == "https:" ? "wss:" : "ws:";
            let url =
                protocol +
                "//" +
                window.location.hostname +
                ":3030/api/websocket";
            console.info("Connecting to ", url);
            this.connection = new WebSocket(url);
            this.connection.onmessage = function (event) {
                let data = JSON.parse(event.data);
                if ("status_list" in data) {
                    console.debug("received status_list reply");
                    this.last_id = data.status_list.last_id;
                    this.events = [...data.status_list.events, ...this.events];
                    this.events = this.events.sort(
                        (a, b) => a.epoch_seconds < b.epoch_seconds
                    );
                    this.awaiting_message = false;
                    this.$store.commit("end_long_poll");
                } else if ("tag_list" in data) {
                    console.debug("received tag_list reply");
                    this.all_tags = data.tag_list.tags;
                    this.awaiting_tags_message = false;
                }
            }.bind(this);

            this.connection.onopen = function (event) {
                console.log(
                    "Successfully connected to the echo websocket server..."
                );

                this.fetchEventsList();
                this.timer = setInterval(this.fetchEventsList, 3000);

                this.refreshTagsList();
                this.tags_timer = setInterval(this.fetchEventsList, 60000);
                this.retry_iteration = 0;
            }.bind(this);

            this.connection.onclose = function () {
                let random_number_milliseconds = Math.floor(
                    Math.random() * 1000
                );
                const maximum_backoff = 64 * 1000;
                let delay = Math.min(
                    Math.pow(2, this.retry_iteration) * 1000 +
                        random_number_milliseconds,
                    maximum_backoff
                );
                console.error(
                    "Failed connection to backend, retrying in " +
                        Math.floor(delay / 1000) +
                        " seconds"
                );
                this.connection = null;
                this.cancelAutoUpdate();

                this.connection_timer = setTimeout(
                    () => this.reconnect(),
                    delay
                );
                this.retry_iteration += 1;
            }.bind(this);
        },
    },
};
</script>
