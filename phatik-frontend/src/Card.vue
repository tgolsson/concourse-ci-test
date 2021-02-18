<template>
    <v-card>
        <v-card-title primary-title>
            <div class="headline">
                {{ event.app }}
            </div>
            <v-spacer></v-spacer>
            <div class="overline">
                <v-tooltip top content-class="date-tooltip">
                    <template v-slot:activator="{ on, attrs }">
                        <time-ago
                            v-bind="attrs"
                            v-on="on"
                            :datetime="new Date(event.epoch_seconds * 1000)"
                            :refresh="60"
                        ></time-ago>
                    </template>
                    <span>{{
                        new Date(event.epoch_seconds * 1000).toLocaleString()
                    }}</span>
                </v-tooltip>
            </div>
        </v-card-title>

        <v-card-text>
            {{ event.message }}
        </v-card-text>
        <v-divider class="mx-4"></v-divider>
        <v-card-text>
            <v-row align="center" class="mx-0">
                <v-chip-group column multiple v-model="local_tags">
                    <v-chip
                        class="caption"
                        v-for="tag in event.tags"
                        v-bind:key="tag"
                        :value="tag"
                        @input="$emit('update:active_tags', temp_tags)"
                        outlined
                        >#{{ tag }}</v-chip
                    ></v-chip-group
                >
            </v-row>
        </v-card-text>
    </v-card>
</template>

<script>
import { TimeAgo } from "vue2-timeago";
export default {
    name: "card",
    data: function () {
        return {
            temp_tags: [],
        };
    },
    components: {
        TimeAgo,
    },
    computed: {
        local_tags: {
            get: function () {
                this.temp_tags = [...this.active_tags];
                return this.temp_tags;
            },
            set: function (nv) {
                this.temp_tags = nv;
                this.$emit("update:active_tags", this.temp_tags);
            },
        },
    },

    emits: ["update:active_tags"],
    props: {
        event: {
            type: Object,
        },
        active_tags: {
            type: Array,
        },
    },
    methods: {
        tagUpdated(x, y, z) {
            this;
        },
    },
};
</script>

<style lang="css">
.date-tooltip {
    transition-duration: 0s !important;
}
</style>