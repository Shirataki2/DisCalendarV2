import { Middleware } from '@nuxt/types'

const myMiddleware: Middleware = ({ store, app, $axios }) => {
  if (app.$cookies.get('access_token')) {
    store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
    store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
    $axios.get('/discord/api/users/@me').then(({ data }) => {
      store.dispatch('auth/setUser', data)
    })
  }
}

export default myMiddleware
