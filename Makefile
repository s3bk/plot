all:
	wasm-pack build --target no-modules
	cp index.html index.js dos/
	cp pkg/plot.js pkg/plot_bg.wasm docs/
	
