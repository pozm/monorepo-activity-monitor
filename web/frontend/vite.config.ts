import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import { base_path } from './config';

export default defineConfig({
  plugins: [solidPlugin()],
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
  server:{
		port:1454,
		proxy:{
			[`${base_path}api`]:{
				target:"http://localhost:9174",
				changeOrigin:true,
				secure:false,
				ws:true,
       		 	rewrite: (p) => p.slice(4)
			}
		}
	},
	base:base_path
});
