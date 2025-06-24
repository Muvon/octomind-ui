import './app.css'
import { mount } from 'svelte'
import App from './App.svelte'

console.log('Main.js loaded, initializing Svelte app...');

const app = mount(App, {
  target: document.getElementById('app'),
})

console.log('Svelte app initialized:', app);

export default app
