<template>
  <div>
    <v-container>
      <v-row>
        <v-col cols="12" lg="10" xl="8" offset-lg="1" offset-xl="2">
          <h1 class="text-h4 mt-4 mb-6">
            {{ page.title }}
          </h1>
          <div style="position: absolute; top: 35px; right: 30px;">
            <v-menu>
              <template #activator="{ on, attrs }">
                <v-btn large icon v-bind="attrs" v-on="on">
                  <v-icon>
                    mdi-menu
                  </v-icon>
                </v-btn>
              </template>
              <v-list width="220">
                <v-virtual-scroll :items="pages" :item-height="50" height="200">
                  <template v-slot="{ item }">
                    <v-list-item :to="item.path" nuxt>
                      <v-list-item-title v-text="item.title" />
                    </v-list-item>
                  </template>
                </v-virtual-scroll>
              </v-list>
            </v-menu>
          </div>
        </v-col>
      </v-row>
      <v-divider class="mb-3" />
      <v-row>
        <v-col cols="12" lg="10" xl="8" offset-lg="1" offset-xl="2">
          <nuxt-content id="docs" :document="page" />
        </v-col>
      </v-row>
      <v-row>
        <v-col
          cols="6"
          sm="5"
          md="4"
          xl="3"
          offset-lg="1"
          offset-xl="2"
        >
          <v-card v-if="prev" outlined :to="prev.path" nuxt>
            <v-card-subtitle>
              前の記事
            </v-card-subtitle>
            <v-card-title>
              <v-icon>
                mdi-chevron-left
              </v-icon>
              <v-spacer />
              {{ prev.title }}
            </v-card-title>
          </v-card>
        </v-col>
        <v-col
          cols="6"
          sm="5"
          md="4"
          xl="3"
          offset-sm="2"
          offset-md="4"
          offset-lg="2"
          offset-xl="2"
        >
          <v-card v-if="next" outlined :to="next.path" nuxt>
            <v-card-subtitle>
              次の記事
            </v-card-subtitle>
            <v-card-title>
              {{ next.title }}
              <v-spacer />
              <v-icon>
                mdi-chevron-right
              </v-icon>
            </v-card-title>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { Vue, Component } from 'vue-property-decorator'
import { Page } from '@/types'

@Component({
  name: 'Support',
  asyncData: async ({ store, app, $axios, $content, params, error }) => {
    const slug = params.slug
    if (!slug) { error({ statusCode: 404, message: 'Page not Found' }) }
    try {
      const page = await $content('docs', slug).fetch()
      const pages = await $content('docs').sortBy('description').fetch()
      if (app.$cookies.get('access_token')) {
      }
      if (store.getters['auth/accessToken'] && store.getters['auth/user']) {
        return { page, pages }
      } else if (app.$cookies.get('access_token')) {
        store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
        store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
        try {
          const { data } = await $axios.get('/discord/api/users/@me')
          store.dispatch('auth/setUser', data)
          return { page, pages }
        } catch (e) {
          return { page, pages }
        }
      }
      return { page, pages }
    } catch {
      error({ statusCode: 404, message: 'Page not Found' })
    }
  }
})
class Support extends Vue {
  page!: Page

  pages: Page[] = []

  get prev () {
    const idx = this.pages.findIndex(page => page.description === this.page.description)
    if (idx > 0) {
      return this.pages[idx - 1]
    } else {
      return null
    }
  }

  get next () {
    const idx = this.pages.findIndex(page => page.description === this.page.description)
    if (idx >= 0 && idx < this.pages.length - 1) {
      return this.pages[idx + 1]
    } else {
      return null
    }
  }
}
export default Support
</script>

<style lang="scss">
#docs {
  h2 {
    margin-top: 20px !important;
    margin-bottom: 10px !important;
  }

  li {
    margin: 6px 1px;
  }

  pre::before {
    content: '';
  }

  pre {
    background: #444 !important;
    code {
      margin: 0 -2px;
    }
  }

  code {
    font-family: 'Courier Prime', monospace !important;
    background: #444 !important;
    color: #fff !important;
    padding: 4px 2px !important;
    margin: 0 4px;
    font-weight: 300 !important;
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
}
</style>
