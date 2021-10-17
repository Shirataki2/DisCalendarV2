import { Middleware } from '@nuxt/types'

const myMiddleware: Middleware = ({ redirect, store, app, $axios }) => {
  if (app.$cookies.get('access_token')) {
    store.dispatch('auth/setAccessToken', app.$cookies.get('access_token'))
    store.dispatch('auth/setRefreshToken', app.$cookies.get('refresh_token'))
    $axios.get('/discord/api/users/@me').then(({ data }) => {
      store.dispatch('auth/setUser', data)
    }).catch(() => {
      redirect(301, '/login')
    })
  } else {
    redirect(301, '/login')
  }
}

export default myMiddleware
