import App from './App.svelte'
import './ts/clipboard'
import './global.scss'

const app = new App({
	target: document.getElementById('app'),
})

export default app
