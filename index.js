async function init() {
    let r = await fetch("pkg/plot_bg.wasm");
    let data = await r.arrayBuffer();
    let module = await wasm_bindgen(data);
    let p = new wasm_bindgen.Plotter();

    function frame(time) {
        p.frame(time);
        window.requestAnimationFrame(frame);
    }
    window.requestAnimationFrame(frame);
}
document.onload = function() {
    init();
};
