<template>
  <v-card class="mb-6 mx-sm-10 mx-md-15 mx-lg-20">
    <v-card-title>{{ name }}</v-card-title>
    <v-divider />
    <v-card-text>
      <v-container class="ma-n4">
        <div class="mt-1 ml-3 mb-4">
          <slot />
        </div>
      </v-container>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator'
@Component({
  name: 'command'
})
class Command extends Vue {
  @Prop({ type: String })
  name!: string
}
export default Command
</script>

<style>
pre::before {
  content: '';
}
code {
  background: #444 !important;
  color: #fff !important;
  padding: 4px 2px !important;
  margin: 0 4px;
}
table {
  width: 100%;
  border-collapse: collapse;
  border-spacing: 0;
}
table th,
table td {
  padding: 10px 0;
  text-align: center;
}
table tr:nth-child(even) {
  background-color: #333;
  color: #ccc;
}
</style>
