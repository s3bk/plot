function f(x, t) {
    return Math.sin(10. * x + t);
}
let PLOT;
async function init() {
    let r = await fetch("pkg/plot_bg.wasm");
    let data = await r.arrayBuffer();
    let module = await wasm_bindgen(data);
    let p = new wasm_bindgen.Plotter(f);
    PLOT = p;
    function frame(time) {
        p.frame(time);
        window.requestAnimationFrame(frame);
    }
    window.requestAnimationFrame(frame);
}
window.onload = function() {
    init();
};

async function test() {
    let data = wasm_bindgen.compile_expr("x + 2 y", "x y");
    let w = await WebAssembly.instantiate(data, {});
    let module = w.module;
    let instance = w.instance;
    let f = instance.exports.f;
    console.log("f(42, 38)", f(42, 38));
}

async function set_expr(expr) {
    let data = wasm_bindgen.compile_expr(expr, "x t");
    let w = await WebAssembly.instantiate(data, {});
    let module = w.module;
    let instance = w.instance;
    let f = instance.exports.f;
    PLOT.set_func(f);
}

