import { defineConfig } from 'vite'

const htmlPlugin = () => {
  return {
    name: 'html-transform',
    transformIndexHtml(html) {
      return html.replace(
        /WEBSITE_BASE_PATH/g,
        process.env.WEBSITE_BASE_PATH || "http://localhost:3000",
      )
    },
  }
}

export default defineConfig({
  // DEV ONLY: Disable SPA history fallback so that requests for nonexistent files (e.g. Bevy
  // .meta sidecar files) return 404 instead of index.html, which would break asset loading.
  // This is safe because we don't use frontend routing.
  appType: 'mpa',
  build: {
    minify: false // temporary, wasm-bindgen js bindings must not be mangled
  },
  plugins: [htmlPlugin()],
  preview: {
    allowedHosts: true, // for ngrok
  }
})
