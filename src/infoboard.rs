
use dioxus::prelude::*;


pub fn InfoBoard(cx: Scope) -> Element {
    render!{
        section {
            class: "info",
            h1 {"Guide"},
            div {
                "InterSpace visualizes the graphical interface pipeline via a dynamic partition graph. To understand it you must first roughly understand the different stages in this pipeline. Here follows a right to left (or platform-to-ui-dev) explanation of the stages."
            },
            div {
                class: "stage_explanation",
                style: "
                    display: grid;
                    grid-template-columns: auto auto;
                    grid-template-rows: repeat(7, auto);
                ",
                div {class: "stage", style: "grid-row: 1 / 2; grid-column: 1 / 2;", div{"platform"}},
                div {class: "info",  style: "grid-row: 1 / 2; grid-column: 2 / 3;",
                    "Genuine platforms are operating systems (e.g. Linux, Android, Windows, MacOs, iOs) which support graphics. Pseudo-platforms include vector canvases like SVG or PDF. On a genuine platform, somewhere in RAM or VRAM lives a framebuffer containing the state of all the pixels to show on screen. Swapping this buffer for a new, different one results in a visible change when the framebuffer is next polled by the display device (i.e. on a fixed rate of e.g. 60Hz or a variable rate with e.g. VSync). Changes to a buffer can be made by a Central Processing Unit (CPU) in so-called software rendering, or by a Graphics Processing Unit (GPU) in so-called hardware rendering. Many problems encountered in graphics lend themselves to parallelization, hence the creation and use of GPUs. While using a CPU, one of few standardized instruction set architectures (ISA) is used, e.g. x86, ARM, RISC-V. These ISAs remain largely constant, hence compilers can compile programs to machine code (instructions from the ISA) to be directly executed. GPUs don't have the same standardized or constant ISAs, i.e. the supported instructions can vary greatly between product generations, and varies greatly between brands."
                },
                div {class: "stage", style: "grid-row: 2 / 3; grid-column: 1 / 2;", div{"gfx"}},
                div {class: "info",  style: "grid-row: 2 / 3; grid-column: 2 / 3;",
                    "To provide a stable and standardized target for execution on GPUs, GPUs are utilized via a driver (generally supplied by the GPU manufacturer) which supports standardized graphics APIs (", i{"Gfxapi, "}, "e.g. Vulkan, OpenGL, Direct3D, Metal) and compute APIs (e.g. CUDA, OpenCL) and translates calls to these APIs to GPU instructions for execution. Compute APIs are not relevant to the topic. Additionally, calls to one graphics API can be translated to calls of another graphics API by graphics API translation layers (", i{"Intergfx"}, "). Some graphics APIs like WebGPU are not supported by drivers, but serve as a standardized cross-platform graphics API which is translated into platform-supported graphics API calls (e.g. WebGPU to Vulkan)."
                },
                div {class: "stage", style: "grid-row: 3 / 4; grid-column: 1 / 2;", div{"raster"}},
                div {class: "info",  style: "grid-row: 3 / 4; grid-column: 2 / 3;",
                    "Converts geometric primitives (vector shapes) to a raster via software or hardware rendering. This stage is skipped when rendering to a vector canvas like SVG or PDF.",
                },
                div {class: "stage", style: "grid-row: 4 / 5; grid-column: 1 / 2;", div{"paint"}},
                div {class: "info",  style: "grid-row: 4 / 5; grid-column: 2 / 3;",
                    "Converts frames into geometric primitives",
                },
                div {class: "stage", style: "grid-row: 5 / 6; grid-column: 1 / 2;", div{"layout"}},
                div {class: "info",  style: "grid-row: 5 / 6; grid-column: 2 / 3;",
                    "Converts widget tree into positioned frames",
                },
                div {class: "stage", style: "grid-row: 6 / 7; grid-column: 1 / 2;", div{"ui"}},
                div {class: "info",  style: "grid-row: 6 / 7; grid-column: 2 / 3;",
                    "supplies ui-developer with means to describe the UI, (content, style, logic), often with declarative UI, reactive state-management, deployment to Web via SSR or Liveview, deployment to native with native renderer or through shell like WebView, embedded browser"
                },
                div {class: "stage", style: "grid-row: 7 / 8; grid-column: 1 / 2;", div{"langbridge"}},
                div {class: "info",  style: "grid-row: 7 / 8; grid-column: 2 / 3;",
                    "language bindings for ui framework/library",
                },
            },
            br{},
            div {
                "Read the graph ",
                div {
                    class: "example_graph",
                    style: r#"
                        display: inline-grid;
                        grid-template-rows: 20px 20px;
                        grid-template-columns: 20px 20px;
                        grid-template-areas: 
                            "A B"
                            "A C"
                        ;
                    "#,
                    div {
                        style: "grid-area: A; background: #f004;",
                        "A"
                    },
                    div {
                        style: "grid-area: B; background: #0f04;",
                        "B"
                    },
                    div {
                        style: "grid-area: C; background: #00f6;",
                        "C"
                    }
                },
                " as \"A can utilize B or C\" or \"A supports using B or C\" or \"B and C are targets of A\""
            }
        }
    }
}