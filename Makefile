all:
	wasm-pack build --target no-modules
	cp index.html index.js doc/
	cp pkg/plot.js pkg/plot_bg.wasm doc/
	
